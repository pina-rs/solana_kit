/** Owns actual Wallet Standard change callbacks, including removal during dispatch. */
function events(state) {
  const listeners = new Set();

  return {
    emit(change) {
      for (const listener of [...listeners]) listener(change);
    },
    count: () => listeners.size,
    feature: {
      version: "1.0.0",
      on(event, listener) {
        validateEvent(event);
        listeners.add(listener);

        return () => unsubscribe(state, listeners, listener);
      },
    },
  };
}

/** Preserves an external unsubscribe exception for the cleanup journal. */
function unsubscribe(state, listeners, listener) {
  state.offAttempts = (state.offAttempts ?? 0) + 1;
  if (state.offFailure) throw state.error;

  listeners.delete(listener);
}

/** Rejects unknown event names before listener registration. */
function validateEvent(event) {
  if (event !== "change") throw new Error("unexpected event");
}

/** Controls a real promise without replacing the production browser adapter. */
function runtime() {
  const state = { mode: "ready", error: undefined, settle: undefined, calls: 0 };
  state.response = async (callback) => {
    state.calls += 1;

    if (state.mode === "reject") throw state.error;
    if (state.mode === "empty") return [];
    if (state.mode === "pending") {
      await new Promise((resolve) => {
        state.settle = resolve;
      });
    }

    return callback();
  };

  return state;
}

/** Rejects correspondence errors at the external wallet boundary. */
function authorize(account, input, transaction = false) {
  if (input.account !== account) throw new Error("foreign account object");

  if (transaction && input.chain !== "solana:localnet") throw new Error("wrong chain");
}

/** Concrete signing callbacks use supplied Ed25519 operations over exact request bytes. */
function signing(account, state, signMessage, signTransaction) {
  return {
    "solana:signMessage": {
      version: "1.0.0",
      async signMessage(...inputs) {
        return state.response(() =>
          inputs.map((input) => {
            authorize(account, input);

            return {
              signedMessage: input.message,
              signature: signMessage(input.message),
              signatureType: "ed25519",
            };
          })
        );
      },
    },
    "solana:signTransaction": {
      version: "1.0.0",
      supportedTransactionVersions: ["legacy", 0, 1],
      async signTransaction(...inputs) {
        return state.response(() =>
          inputs.map((input) => {
            authorize(account, input, true);

            return { signedTransaction: signTransaction(input.transaction) };
          })
        );
      },
    },
    "solana:signAndSendTransaction": {
      version: "1.0.0",
      supportedTransactionVersions: ["legacy", 0, 1],
      async signAndSendTransaction(...inputs) {
        return state.response(() => inputs.map((input) => {
          authorize(account, input, true);
          const signed = signTransaction(input.transaction);

          return { signature: state.submissionSignature ?? signed.slice(1, 65) };
        }));
      },
    },
  };
}

/** Authorization publishes account events before the connect promise resolves. */
function connection(wallet, account, emitter) {
  return {
    "standard:connect": {
      version: "1.0.0",
      async connect() {
        wallet.accounts = [account];
        emitter.emit({ accounts: wallet.accounts });

        return { accounts: wallet.accounts };
      },
    },
    "standard:disconnect": {
      version: "1.0.0",
      async disconnect() {
        wallet.accounts = [];
        emitter.emit({ accounts: [] });
      },
    },
  };
}

/** Installs the protocol handshake used by real Window wallet extensions. */
function announce(wallet) {
  let unregister;
  const register = (api) => {
    const next = api.register(wallet);
    unregister ??= next;
  };
  const ready = (event) => register(event.detail);
  window.addEventListener("wallet-standard:app-ready", ready);

  return {
    announce() {
      window.dispatchEvent(
        new CustomEvent("wallet-standard:register-wallet", {
          detail: register,
        }),
      );
    },
    close() {
      window.removeEventListener("wallet-standard:app-ready", ready);
      unregister?.();
    },
  };
}

/** A concrete Window wallet exercises discovery, events and real signing requests. */
export function fixture(address, publicKey, signMessage, signTransaction) {
  const account = {
    address,
    publicKey,
    chains: ["solana:localnet"],
    features: ["solana:signMessage", "solana:signTransaction", "solana:signAndSendTransaction"],
  };
  const state = runtime();
  const emitter = events(state);
  const wallet = {
    version: "1.0.0",
    name: "Kit protocol test wallet",
    icon: "data:image/png;base64,",
    chains: ["solana:localnet"],
    accounts: [],
    features: {},
  };
  wallet.features = {
    ...connection(wallet, account, emitter),
    "standard:events": emitter.feature,
    ...signing(account, state, signMessage, signTransaction),
  };

  return {
    ...announce(wallet),
    wallet,
    runtime: state,
    revoke() {
      wallet.accounts = [];
      emitter.emit({ accounts: [] });
    },
    restore() {
      wallet.accounts = [account];
      emitter.emit({ accounts: wallet.accounts });
    },
    listeners: emitter.count,
    malformed() {
      emitter.emit({ accounts: 42 });
    },
    settle() {
      state.settle?.();
    },
    unsupported() {
      wallet.features = {
        ...wallet.features,
        "solana:signMessage": { ...wallet.features["solana:signMessage"], version: "2.0.0" },
      };
      emitter.emit({ features: wallet.features });
    },
  };
}
