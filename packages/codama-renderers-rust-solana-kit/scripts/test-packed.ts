import { deepStrictEqual } from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { preparePackedConsumer } from "./prepare-packed.ts";
import { verifyGeneratedClient } from "./verify-client.ts";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const metadata: unknown = JSON.parse(readFileSync(join(packageDirectory, "package.json"), "utf8"));

if (
  typeof metadata !== "object" || metadata === null || !("name" in metadata)
  || !("version" in metadata) || typeof metadata.name !== "string"
  || typeof metadata.version !== "string"
  || !("devDependencies" in metadata) || typeof metadata.devDependencies !== "object"
  || metadata.devDependencies === null || !("codama" in metadata.devDependencies)
  || typeof metadata.devDependencies.codama !== "string"
) {
  throw new Error("Invalid NPM package identity");
}

const archive = process.argv[2]
  ?? join(packageDirectory, `${metadata.name}-${metadata.version}.tgz`);
const consumer = mkdtempSync(join(tmpdir(), "solana-kit-packed-"));

try {
  preparePackedConsumer({
    packageDirectory,
    consumer,
    archive,
    name: metadata.name,
    version: metadata.version,
    codama: metadata.devDependencies.codama,
  });
  execFileSync("npm", [
    "ci",
    "--offline",
    "--ignore-scripts",
    "--no-audit",
    "--no-fund",
    "--user-agent=solana-kit-package-verification",
  ], { cwd: consumer, stdio: "inherit" });

  const source = readFileSync(join(packageDirectory, "test/packed-consumer.ts"), "utf8");
  writeFileSync(join(consumer, "verify.ts"), source);

  for (const file of ["contracts.ts", "fixture.ts", "numeric.json"]) {
    writeFileSync(join(consumer, file), readFileSync(join(packageDirectory, "test", file)));
  }

  const env = { ...process.env, PATH: "", CARGO: "", RUSTC: "", RUSTDOC: "" };
  execFileSync(process.execPath, ["verify.ts"], { cwd: consumer, stdio: "inherit", env });
  writeFileSync(
    join(consumer, "codama.json"),
    JSON.stringify({
      idl: "root.json",
      scripts: { rust: { from: metadata.name, args: ["cli-generated"] } },
    }),
  );
  execFileSync(process.execPath, [
    "node_modules/codama/bin/cli.cjs",
    "--config",
    "codama.json",
    "run",
    "rust",
  ], { cwd: consumer, stdio: "inherit", env });

  for (
    const file of [
      "counter.rs",
      "empty_pda_contracts.rs",
      "literal_contracts.rs",
      "numeric_contracts.rs",
      "record_contracts.rs",
      "resolver_contracts.rs",
      "mod.rs",
    ]
  ) {
    deepStrictEqual(
      readFileSync(join(consumer, "generated", file)),
      readFileSync(join(consumer, "cli-generated", file)),
    );
  }

  verifyGeneratedClient(consumer);
} finally {
  rmSync(consumer, { recursive: true });
}
