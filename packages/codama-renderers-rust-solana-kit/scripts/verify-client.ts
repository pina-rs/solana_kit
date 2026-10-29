import { equal } from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { cpSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** Runs the maintained client against this package's actual generated source. */
export function verifyGeneratedClient(consumer: string): void {
  const cargo = process.env.CARGO;
  const rustc = process.env.RUSTC;
  const rustdoc = process.env.RUSTDOC;

  if (!cargo || !rustc || !rustdoc) {
    throw new Error("Provide concrete CARGO, RUSTC and RUSTDOC for the generated client");
  }

  const repository = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
  const client = prepareClient(consumer, repository);
  const command = (args: string[]) =>
    execFileSync(cargo, args, {
      cwd: client,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "inherit"],
    });

  execFileSync(rustc, ["--version", "--verbose"], { stdio: "inherit" });
  command(["generate-lockfile", "--offline"]);
  const output = command(["run", "--offline", "--locked", "--bin", "client"]);
  equal(output.trim(), "generated-client-contracts:9");
  const lintArguments = [
    "clippy",
    "--verbose",
    "--offline",
    "--locked",
    "--all-targets",
    "--",
    "-D",
    "warnings",
  ];
  command(lintArguments);
}

function prepareClient(consumer: string, repository: string): string {
  const fixture = resolve(repository, "verification/codama-consumer");
  const client = resolve(consumer, "client");
  for (
    const file of [
      "Cargo.toml",
      "src/lib.rs",
      "src/contracts.rs",
      "src/numeric.rs",
      "src/main.rs",
      "guest/Cargo.toml",
      "guest/src/lib.rs",
      "host/Cargo.toml",
      "host/src/main.rs",
    ]
  ) {
    const destination = resolve(client, file);
    mkdirSync(dirname(destination), { recursive: true });
    cpSync(resolve(fixture, file), destination);
  }
  mkdirSync(resolve(client, "generated"), { recursive: true });
  for (
    const file of [
      "counter.rs",
      "empty_pda_contracts.rs",
      "literal_contracts.rs",
      "numeric_contracts.rs",
      "record_contracts.rs",
      "resolver_contracts.rs",
    ]
  ) {
    cpSync(resolve(consumer, "generated", file), resolve(client, "generated", file));
  }

  const core = resolve(repository, "crates/solana_kit_core");
  const manifest = readFileSync(resolve(client, "Cargo.toml"), "utf8")
    .replace("\"../../crates/solana_kit_core\"", JSON.stringify(core));
  writeFileSync(resolve(client, "Cargo.toml"), manifest);

  return client;
}
