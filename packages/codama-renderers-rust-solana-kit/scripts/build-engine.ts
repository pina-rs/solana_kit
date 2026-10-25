import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { isEngineArtifact } from "./engine-artifact.ts";

const packageDirectory = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repository = resolve(packageDirectory, "../..");
const cargo = process.env.CARGO ?? "cargo";
const inherited = process.env.CARGO_ENCODED_RUSTFLAGS;

if (!inherited && process.env.RUSTFLAGS?.trim()) {
  throw new Error("Use CARGO_ENCODED_RUSTFLAGS when building the distributable engine");
}

const flags = inherited ? inherited.split("\u001f") : [];
flags.push(`--remap-path-prefix=${repository}=/solana-kit`);
flags.push(`--remap-path-prefix=${homedir()}=/user`);
const output = execFileSync(cargo, [
  "build",
  "--locked",
  "--offline",
  "--manifest-path",
  "tools/codama-renderer/Cargo.toml",
  "--release",
  "--target",
  "wasm32-wasip1",
  "--no-default-features",
  "--bin",
  "solana_kit_codegen_wasi",
  "--message-format=json-render-diagnostics",
], {
  cwd: repository,
  encoding: "utf8",
  stdio: ["ignore", "pipe", "inherit"],
  env: { ...process.env, CARGO_ENCODED_RUSTFLAGS: flags.join("\u001f") },
});
const artifacts = new Set<string>();
const expected = {
  manifestPath: resolve(repository, "tools/codama-renderer/Cargo.toml"),
  sourcePath: resolve(repository, "tools/codama-renderer/src/bin/solana_kit_codegen_wasi.rs"),
  targetName: "solana_kit_codegen_wasi",
};

for (const line of output.split("\n").filter(Boolean)) {
  const message: unknown = JSON.parse(line);

  if (isEngineArtifact(message, expected)) {
    artifacts.add(message.executable);
  }
}

if (artifacts.size !== 1) {
  throw new Error("Cargo must report exactly one executable for the WASI renderer");
}

const artifact = artifacts.values().next().value;

if (artifact === undefined) {
  throw new Error("Cargo did not report the WASI renderer artifact");
}

mkdirSync(resolve(packageDirectory, "engine"), { recursive: true });
copyFileSync(artifact, resolve(packageDirectory, "engine/renderer.wasm"));
