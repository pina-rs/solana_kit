//! Native controls exercise real updates, acknowledged cancellation and stale callbacks.
mod activity;
mod delivery;
mod panel;
mod transport;

use super::{
    Action, AppError, Error, LocalWalletCommands, LocalWalletEvents, REQUEST_TIMEOUT_MILLIS,
    Routing, Rpc, bounded, read_config, show, sleep,
};
use activity::Activity;
use delivery::{Delivery, Update};
use futures::future::Either;
use panel::Command;
use solana_kit_client::methods::Balance;
use solana_kit_client::subscriptions::Drain;
use transport::Transport;

enum Phase {
    Opening,
    Live,
    Armed,
    Held(Update),
    Cancelled,
}

struct View {
    delivery: Delivery,
    activity: Activity,
    phase: Phase,
}

impl View {
    fn status(&self) -> &'static str {
        match self.phase {
            Phase::Opening => "Opening subscription socket",
            Phase::Live => "Account subscription active",
            Phase::Armed => "Waiting to hold a real subscription update",
            Phase::Held(_) => "Real subscription update held",

            Phase::Cancelled if matches!(self.activity, Activity::Observed { .. }) => {
                "Same-socket slot activity observed"
            }

            Phase::Cancelled if matches!(self.activity, Activity::Waiting { .. }) => {
                "Waiting for later same-socket slot"
            }

            Phase::Cancelled => "Held update ignored after cancellation",
        }
    }

    /// Each synchronous drain is bounded by the SDK queue budget. Both queues
    /// are consumed even when no redraw is needed; the UI gets first poll of
    /// its action channel before the next socket receive can admit more work.
    fn drain(&mut self, transport: &mut Transport) -> Result<bool, AppError> {
        let mut redraw = false;

        while let Drain::Notification(notification) = transport.owner.next(&transport.account)? {
            let update = Update {
                generation: transport.generation,
                lamports: notification.value.require()?.lamports,
            };

            self.accept(update)?;
            redraw = true;
        }

        while let Drain::Notification(notification) = transport.owner.next(&transport.slots)? {
            redraw |= self.activity.observe(notification.slot);
        }

        Ok(redraw)
    }

    fn accept(&mut self, update: Update) -> Result<(), AppError> {
        if matches!(self.phase, Phase::Armed | Phase::Held(_)) {
            self.phase = Phase::Held(update);

            return Ok(());
        }

        if !self.delivery.apply(update) {
            return Err(Error::Subscription("Unexpected stale subscription delivery").into());
        }

        Ok(())
    }

    async fn command(
        &mut self,
        command: Command,
        transport: &mut Transport,
        routing: &mut Routing<'_>,
        client: &Rpc,
        actions: &mut LocalWalletEvents,
    ) -> Result<bool, AppError> {
        match (command, &self.phase) {
            (Command::Hold, Phase::Live) if self.delivery.balance.is_some() => {
                self.phase = Phase::Armed;
            }

            (Command::Cancel, Phase::Held(update)) => {
                let held = *update;

                self.cancel(held, transport, routing).await?;
            }

            (Command::Observe, Phase::Cancelled) if matches!(self.activity, Activity::Idle) => {
                self.observe_activity(client, actions).await?;
            }

            (Command::Close, _) => return Ok(true),
            _ => {
                return Err(Error::Subscription("Subscription action is no longer current").into());
            }
        }

        Ok(false)
    }

    async fn observe_activity(
        &mut self,
        client: &Rpc,
        actions: &mut LocalWalletEvents,
    ) -> Result<(), AppError> {
        let observed = bounded(
            client.call(Balance(transport::ACCOUNT, read_config())),
            actions,
        )
        .await?;
        self.activity = Activity::Waiting {
            context: observed.context.slot,
            balance: observed.value,
        };
        Ok(())
    }

    /// Release a genuine callback after remote cleanup, so local generation
    /// rejection stays observable when the server correctly stops sending.
    async fn cancel(
        &mut self,
        held: Update,
        transport: &mut Transport,
        routing: &mut Routing<'_>,
    ) -> Result<(), AppError> {
        self.delivery.cancel();
        timed(transport.cancel_account(routing)).await?;

        if self.delivery.apply(held) {
            return Err(
                Error::Subscription("Cancelled update changed the displayed balance").into(),
            );
        }

        self.phase = Phase::Cancelled;
        Ok(())
    }
}

pub(super) async fn run(
    address: &str,
    generation: u64,
    routing: &mut Routing<'_>,
    actions: &mut LocalWalletEvents,
    wallet_commands: &LocalWalletCommands,
    client: &Rpc,
) -> Result<(), AppError> {
    let mut transport = Transport::new(generation)?;
    let result = journey(
        address,
        &mut transport,
        routing,
        actions,
        wallet_commands,
        client,
    )
    .await;
    let cleanup = transport.close(routing).await;
    finish(result, cleanup)
}

/// UI intents cancel only their pending receive borrow; the application keeps
/// queued events and routes continuing slot updates through the same socket.
async fn journey(
    address: &str,
    transport: &mut Transport,
    routing: &mut Routing<'_>,
    actions: &mut LocalWalletEvents,
    wallet_commands: &LocalWalletCommands,
    client: &Rpc,
) -> Result<(), AppError> {
    let (commands, mut input) = panel::channel();
    let mut view = View {
        delivery: Delivery::new(transport.generation),
        activity: Activity::Idle,
        phase: Phase::Opening,
    };
    show(
        panel::view(address, &view, wallet_commands, &commands),
        view.status(),
    );
    let startup = timed(transport.start(routing));
    let action = panel::next(actions, &mut input);

    match futures::future::select(Box::pin(startup), Box::pin(action)).await {
        Either::Left((result, _)) => result?,
        Either::Right((Ok(Command::Close), _)) => return Ok(()),
        Either::Right((Err(error), _)) => return Err(error),
        _ => return Err(Error::Subscription("Subscription is still opening").into()),
    }

    view.phase = Phase::Live;
    let mut redraw = true;

    loop {
        redraw |= view.drain(transport)?;

        if redraw {
            show(
                panel::view(address, &view, wallet_commands, &commands),
                view.status(),
            );
            redraw = false;
        }

        let action = panel::next(actions, &mut input);
        let event = transport.receive(routing);

        let command = match futures::future::select(Box::pin(action), Box::pin(event)).await {
            Either::Right((result, _)) => {
                result?;
                continue;
            }
            Either::Left((command, _)) => command?,
        };

        if view
            .command(command, transport, routing, client, actions)
            .await?
        {
            return Ok(());
        }

        redraw = true;
    }
}

/// Cleanup cannot be interrupted by another UI cancel, but its real native wait is finite.
async fn timed<T>(
    operation: impl std::future::Future<Output = Result<T, AppError>>,
) -> Result<T, AppError> {
    match futures::future::select(Box::pin(operation), Box::pin(sleep(REQUEST_TIMEOUT_MILLIS)))
        .await
    {
        Either::Left((result, _)) => result,
        Either::Right((timer, _)) => {
            timer?;
            Err(Error::Deadline.into())
        }
    }
}

fn finish<T>(result: Result<T, AppError>, cleanup: Result<(), AppError>) -> Result<T, AppError> {
    match (result, cleanup) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), Ok(())) | (Ok(_), Err(error)) => Err(error),
        (Err(source), Err(cleanup)) => Err(Error::SubscriptionCleanup { source, cleanup }.into()),
    }
}
