<!-- {@signerRegistrationDocs} -->
An explicit supported-interface declaration borrowing one logical capability.

Only declared interfaces are eligible for role selection. Constructors require
the corresponding backend traits; absent interfaces have no backend adapter.
<!-- {/signerRegistrationDocs} -->

<!-- {@messageRegistrationDocs} -->
One explicit message interface borrowing a caller-owned logical capability.

Register a composite backend in exactly one role. Distinct handles claiming an
address conflict, including handles borrowing coincident zero-sized backends.
<!-- {/messageRegistrationDocs} -->

<!-- {@rendererSchemaModuleDocs} -->
Source-table lowering reuses the common Codama model and its closed validation.
<!-- {/rendererSchemaModuleDocs} -->

<!-- {@rendererWireDocs} -->
This small source vocabulary is not a second wire codec or emitter.
<!-- {/rendererWireDocs} -->

<!-- {@publisherInstall} -->
cargo install --git https://github.com/pina-rs/solana_kit solana_kit_dapp_publisher_cli
<!-- {/publisherInstall} -->

<!-- {@publisherInstallLocal} -->
cargo install --path crates/solana_kit_dapp_publisher_cli
<!-- {/publisherInstallLocal} -->

<!-- {@pythAccumulatorExample} -->
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_pyth::{
    PythPriceFeedMessage, parse_accumulator_update_data, parse_pyth_price_feed_message,
};

fn first_price(data: &[u8], limits: Limits) -> Result<PythPriceFeedMessage, Error> {
    let update = parse_accumulator_update_data(data, limits)?;
    let message = update.updates.first().ok_or(Error::InvalidValue)?.message;
    parse_pyth_price_feed_message(message, limits)
}
<!-- {/pythAccumulatorExample} -->

<!-- {@pythReceiverExample} -->
use solana_kit_core::{Address, Instruction, codecs::{Error, Limits}};
use solana_kit_pyth::{
    MerklePriceUpdate, PostUpdateParams, ReceiverAccounts, ReceiverInstruction,
};

fn posted(
    accounts: ReceiverAccounts,
    encoded_vaa: Address,
    update: &MerklePriceUpdate<'_>,
) -> Result<Instruction, Error> {
    ReceiverInstruction::PostUpdate {
        accounts,
        encoded_vaa,
        params: PostUpdateParams::new(update),
    }
    .instruction(Limits::new(4096, 64))
}
<!-- {/pythReceiverExample} -->

<!-- {@surfpoolFundExample} -->
use solana_kit_client::{Client, Transport};
use solana_kit_core::Address;
use solana_kit_surfpool::{Surfnet, builders::SetAccount};
use solana_kit_surfpool::errors::SurfpoolRpcError;

async fn fund<T: Transport>(rpc: Client<T>, address: Address)
    -> Result<(), SurfpoolRpcError>
{
    let surfnet = Surfnet::new(rpc);
    surfnet.execute(SetAccount::new(address).with_lamports(10_000_000_000)).await?;
    Ok(())
}
<!-- {/surfpoolFundExample} -->

<!-- {@jupiterSearchExample} -->
use solana_kit_client::rest::{Budgets, Transport};
use solana_kit_jupiter::{Client, Error, Service, TokenItem};

async fn search<T: Transport<Service>>(transport: T) -> Result<Vec<TokenItem>, Error> {
    let client = Client::new(transport, Budgets::default());
    client.search_tokens("SOL", Some(10)).await
}
<!-- {/jupiterSearchExample} -->

<!-- {@jupiterQuoteExample} -->
use solana_kit_client::rest::{Budgets, Transport};
use solana_kit_core::Address;
use solana_kit_jupiter::{Client, Error, Integer, OrderRequest, OrderResponse, Service};

async fn quote<T: Transport<Service>>(
    transport: T,
    input_mint: Address,
    output_mint: Address,
) -> Result<OrderResponse, Error> {
    let request = OrderRequest::new(input_mint, output_mint, Integer::from(1_000_000));
    Client::new(transport, Budgets::default()).get_order(&request).await
}
<!-- {/jupiterQuoteExample} -->

<!-- {@publisherAttestationExample} -->
use solana_kit_core::{Hash, codecs::Limits};
use solana_kit_dapp_publisher::{AttestationMessage, RequestId};

let request_id = RequestId::parse("00000000000000000000000000000042")?;
let message = AttestationMessage::new(100, Hash::from_bytes([0; 32]), request_id,
    Limits::new(1024, 4096))?;
// Present message.bytes() to the application's approved Ed25519 signer.
# Ok::<(), solana_kit_dapp_publisher::Error>(())
<!-- {/publisherAttestationExample} -->

<!-- {@clientDependencyExample} -->
[dependencies]
solana_kit_core = { git = "https://github.com/pina-rs/solana_kit", features = ["crypto"] }
solana_kit_client = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_native = { git = "https://github.com/pina-rs/solana_kit" }
solana_kit_system = { git = "https://github.com/pina-rs/solana_kit" }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
<!-- {/clientDependencyExample} -->
