# Native Surfpool interop verification

This private workspace binary exercises stock Surfpool **1.7.0** through the
current `NativeHttp`, `Client` and typed `Surfnet`. It adds no public SDK API and
does not install, download or select a binary implicitly.

Root must independently verify the executable, provide a fresh private runtime
path, a fresh base58 genesis hash, a fresh 16–64 character alphanumeric nonce,
three distinct nonzero ports, and a macOS sandbox profile. Run the built harness
under an admitted outer process-group controller:

```text
solana_kit_surfpool_native_verification BIN PROFILE NEW_RUNTIME GENESIS NONCE RPC WS STUDIO [--body-cap]
```

The profile must deny outbound network except explicit loopback, and deny real
home/private-key access. It must allow the verified executable, OS dependencies,
the private runtime. The Surfpool child executable is
wrapped in `/usr/bin/sandbox-exec -f PROFILE`. The environment is cleared, `HOME`
is omitted, and only system `PATH`, private `TMPDIR`, `XDG_CONFIG_HOME` and the
explicit telemetry opt-out are supplied. `NO_DNA` and `--offline` **do not** stop
1.7.0's unconditional cloud version request; privacy depends on the supplied
boundary. Do not relax that boundary to make startup pass.

Startup uses the portable argument builder plus verified `--no-deploy`, in-memory
database, genesis, explicit keypair and log settings. The payer is the public,
deterministic test seed `[7; 32]`, never a real wallet. Kit derives/imports both
key halves and verifies an actual signature; secret buffers and signer storage
zeroize on drop. Its private JSON file is mode 0600. The exclusively created
runtime is mode 0700. Parent-provided freshness and immutable executable/profile
selection are assumptions, not authenticated by a CLI path check.

The positive journey requires a matching fresh `getGenesisHash`, actual
`surfnet_getSurfnetInfo` startup readiness with no planning/task error, and live
kernel child/PGID observations. It verifies startup funding, exact lamports and
binary account readback, pause/time travel/processed epoch read/resume, then awaited exit.
The clock mutation confirms the open slot; a default finalized read can remain
behind while block production is paused. Both mutation and processed read must
match the exact requested slot.
Normal RPC has a 64 KiB explicit wire limit and two-second complete exchanges.
Startup polls for 30 seconds; bounded observations/exchanges can finish after
that polling deadline. Ports are caller-assigned, not reserved by probing.
RPC readiness establishes neither WebSocket delivery nor Studio port ownership.

`--body-cap` sends a typed `WriteProgram` with 8 MiB raw bytes, expanded to over
15 MiB by hexadecimal JSON, using an explicit 18 MiB SDK wire budget. It requires
actual HTTP 413 and proves the selected nonexistent account remains absent.
A local SDK body-budget failure, JSON-RPC error, transport reset or unexpected
success fails the probe. It uses no `.so` or downloaded asset. The public deploy
default uses 3 KiB raw chunks to fit the default client's 8 KiB complete wire
budget. Explicit oversized chunks remain errors rather than being silently resized.

The retained `std::process::Child` is never `try_wait`-reaped during liveness
checks. No new PGID, Tokio process owner, signal-by-port or foreign-PID kill is
used. Confirmed shutdown observes exit with `waitid(WNOWAIT)` without consuming
the status, then waits/reaps and joins bounded log
readers and only then removes the owned runtime after a successful journey.
Failed journeys and unsuccessful child exits retain bounded diagnostic logs after
reaping for root inspection. A nonzero exit or forced kill cannot produce PASS.
Stdout/stderr share a hard
512 KiB file cap. Child-internal log files are disabled by CLI level, but root's
resource controller must still bound the complete runtime; this is not a disk
quota. The shutdown stages and trusted observations have finite deadlines;
kernel scheduling and I/O are not hard real-time guarantees.

`CleanupUnconfirmed` leaves the handle/runtime intact until terminal harness
failure. The mandatory outer controller must drain the inherited group. This is
an explicit private verification failure protocol, **not** a production shutdown
guarantee. Drop creates no task/runtime and does not pretend cleanup succeeded.
Default SIGCHLD disposition, no other child waiter and `sandbox-exec` exec
behavior must be established by root. The kernel rejects nonchildren; `getpgid`
also verifies the inherited process group. No `ps` subprocess is required:
macOS refuses to execute its setuid system binary inside this sandbox.
Preparation or
spawn failures can leave a new runtime for root inspection/removal.

Before adoption, root must generate and verify the lockfile offline under fresh
compiler admission, compile the new member, run strict lint
and the real binary journey, then exercise these independent controls:

| Control                                | Required observation                                                                         |
| -------------------------------------- | -------------------------------------------------------------------------------------------- |
| Unrelated healthy listener on RPC port | Fresh genesis mismatch or owned child exit; never send that listener `exit`                  |
| Early-exiting executable fixture       | Nonzero journey, retained child reaped, no readiness acceptance                              |
| Log-flooding fixture                   | File stays within 512 KiB; failure triggers bounded owned shutdown                           |
| Fixture ignoring RPC shutdown          | Escalation targets only the retained child; observed exit and reap precede directory removal |
| Failed exit observation or held pipe   | Typed cleanup failure, runtime retained, outer group drained                                 |
| Sandbox denial/cloud version stall     | Bounded startup failure and cleanup; no boundary bypass                                      |

These are source requirements and seams, not executed control results. WS,
signed System transfers, automatic ports and public native lifecycle APIs remain
next-phase work.

Two local runs against stock 1.7.0 observed fresh genesis and readiness, funded
payer and exact account bytes, confirmed time travel and processed-bank readback,
HTTP 413 with an absent account after an 8 MiB raw write, and successful exit
followed by reaping. A later strict oversized-request replay failed after the
startup, account and clock checks. Its typed cause remains undiagnosed; that
replay is not accepted as a pass or evidence that the optional probe is reliable.
Current-source CI remains a separate required check. The Darwin arm64 executable
used in the two successful runs had verified SHA-256
`64c0e114e508cc9d61a106d2d0e80f1865e901f6bca2a4546b0363b6ffd79934`.

`cargo test --locked -p solana_kit_surfpool_native_verification` exercises
nonchild rejection, retained kernel exit status, late log overflow, and nonzero
or forced termination with reaping and retained diagnostics. These checks cover
the private verifier; they do not establish all cheatcodes or the public host API.

For that phase, prefer an optional `surfpool` feature in existing
`solana_kit_native`, depending optionally on the portable crate:

```text
spawn(config, verified_tool, boundary, runtime) -> Result<OwnedSurfpool>
attach(http, subscriptions, &caller_signer) -> AttachedSurfpool
OwnedSurfpool::close(&mut self).await -> Result<CloseReceipt, CloseError>
```

Keep owned spawn and borrowed attachment distinct. A close timeout must retain
the owner and runtime for explicit retry or caller containment. Bounded log
snapshots and explicit artifact-directory discovery should be caller-selected;
no ambient project search, implicit download or detached cleanup is justified.
