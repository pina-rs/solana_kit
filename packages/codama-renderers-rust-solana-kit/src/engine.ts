import type { RootNode } from "@codama/nodes";
import { closeSync, openSync, readFileSync, writeFileSync } from "node:fs";
import { mkdtempSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { WASI } from "node:wasi";
import { parseFiles, type RustRenderMap } from "./files.js";
import { getWasiImports } from "./imports.js";

const maxInputBytes = 8 * 1024 * 1024;
const maxOutputBytes = 64 * 1024 * 1024;
let engineModule: WebAssembly.Module | undefined;

/** Runs the bundled trusted engine with isolated stdio and no inherited env or preopens. */
export function render(root: RootNode): RustRenderMap {
  const input = JSON.stringify(root);

  if (Buffer.byteLength(input) > maxInputBytes) {
    throw new Error("Codama JSON exceeds 8 MiB");
  }

  engineModule ??= loadModule();
  const directory = mkdtempSync(join(tmpdir(), "solana-kit-render-"));
  const descriptors: number[] = [];

  try {
    return execute(engineModule, input, directory, descriptors);
  } finally {
    for (const descriptor of descriptors) {
      closeSync(descriptor);
    }

    rmSync(directory, { recursive: true });
  }
}

function loadModule(): WebAssembly.Module {
  const entry = import.meta.resolve("codama-renderers-rust-solana-kit");
  const filename = new URL("../engine/renderer.wasm", entry);
  const bytes = Uint8Array.from(readFileSync(filename));

  return new WebAssembly.Module(bytes);
}

/** Owns one invocation's descriptors; the module itself contains no per-call state. */
function execute(
  module: WebAssembly.Module,
  input: string,
  directory: string,
  descriptors: number[],
): RustRenderMap {
  const inputPath = join(directory, "input.json");
  const outputPath = join(directory, "output.json");
  const errorPath = join(directory, "error.txt");
  writeFileSync(inputPath, input, { flag: "wx", mode: 0o600 });
  const stdin = openSync(inputPath, "r");
  descriptors.push(stdin);
  const stdout = openSync(outputPath, "wx", 0o600);
  descriptors.push(stdout);
  const stderr = openSync(errorPath, "wx", 0o600);
  descriptors.push(stderr);
  const wasi = new WASI({
    version: "preview1",
    args: [],
    env: {},
    preopens: {},
    returnOnExit: true,
    stdin,
    stdout,
    stderr,
  });
  const instance = new WebAssembly.Instance(module, getWasiImports({ module, wasi }));
  const status = wasi.start(instance);

  if (status !== 0) {
    throw new Error(
      `Rust renderer rejected the IDL: ${readFileSync(errorPath, "utf8").slice(0, 8192)}`,
    );
  }

  if (statSync(outputPath).size > maxOutputBytes) {
    throw new Error("Renderer response exceeds 64 MiB");
  }

  const response: unknown = JSON.parse(readFileSync(outputPath, "utf8"));

  return parseFiles(response);
}
