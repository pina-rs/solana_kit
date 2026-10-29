import { visit } from "@codama/visitors-core";
import { createFromRoot } from "codama";
import { deepStrictEqual, equal, throws } from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { parseFiles } from "../src/files.js";
import { getRenderMapVisitor, renderVisitor } from "../src/index.js";
import { writeFiles } from "../src/write.js";
import { contractRoot } from "./contracts.ts";
import { counterRoot } from "./fixture.js";

test("complete record and resolver contracts pass the normal Codama boundary", () => {
  const files = createFromRoot(contractRoot()).accept(getRenderMapVisitor());
  deepStrictEqual([...files.keys()].sort(), [
    "counter.rs",
    "empty_pda_contracts.rs",
    "literal_contracts.rs",
    "mod.rs",
    "numeric_contracts.rs",
    "record_contracts.rs",
    "resolver_contracts.rs",
  ]);
});

test("actual Rust engine returns deterministic normal Codama fragments", () => {
  const root = counterRoot();
  const first = visit(root, getRenderMapVisitor());
  deepStrictEqual([...first], [...visit(root, getRenderMapVisitor())]);
  equal(first.has("counter.rs"), true);
  equal(first.has("mod.rs"), true);
});

test("normal Codama factory accepts the public render-map visitor", () => {
  const root = counterRoot();
  const codama = createFromRoot(root);
  deepStrictEqual(
    [...codama.accept(getRenderMapVisitor())],
    [...visit(root, getRenderMapVisitor())],
  );
});

test("the real Codama CLI loads the default renderer and writes the same modules", () => {
  const directory = mkdtempSync(join(tmpdir(), "solana-kit-cli-"));
  const root = counterRoot();
  const cli = join(dirname(fileURLToPath(import.meta.resolve("codama"))), "../bin/cli.cjs");
  const renderer = fileURLToPath(new URL("../../dist/index.js", import.meta.url));

  try {
    writeFileSync(join(directory, "root.json"), JSON.stringify(root));
    const config = {
      idl: "root.json",
      scripts: {
        rust: { from: renderer, args: [join(directory, "generated")] },
      },
    };
    const configuration = join(directory, "codama.json");
    writeFileSync(configuration, JSON.stringify(config));
    execFileSync(process.execPath, [cli, "--config", configuration, "run", "rust"], {
      cwd: directory,
      env: { ...process.env, PATH: "" },
      stdio: "pipe",
    });
    const expected = visit(root, getRenderMapVisitor()).get("counter.rs")?.content;
    equal(readFileSync(join(directory, "generated/counter.rs"), "utf8"), expected);
  } finally {
    rmSync(directory, { recursive: true });
  }
});

test("regeneration replaces generated modules and preserves unrelated files", () => {
  const directory = mkdtempSync(join(tmpdir(), "kit-render-test-"));

  try {
    writeFileSync(join(directory, "notes.txt"), "caller notes");
    visit(counterRoot(), renderVisitor(directory));
    const expected = readFileSync(join(directory, "counter.rs"), "utf8");
    writeFileSync(join(directory, "counter.rs"), "old generated content");
    visit(counterRoot(), renderVisitor(directory));
    equal(readFileSync(join(directory, "counter.rs"), "utf8"), expected);
    equal(readFileSync(join(directory, "notes.txt"), "utf8"), "caller notes");
  } finally {
    rmSync(directory, { recursive: true });
  }
});

test("response validation rejects escaping and duplicate modules", () => {
  throws(() => parseFiles([{ path: "../outside.rs", source: "" }]));
  throws(() => parseFiles([{ path: "/outside.rs", source: "" }]));
  throws(() => parseFiles([{ path: "x.rs", source: "" }, { path: "x.rs", source: "" }]));
  throws(() => parseFiles([{ path: "mod.rs", source: "", unexpected: true }]));
});

test("destination symlinks fail before any generated module is overwritten", () => {
  const directory = mkdtempSync(join(tmpdir(), "kit-path-test-"));

  try {
    const outside = join(directory, "outside.txt");
    writeFileSync(outside, "caller bytes");
    symlinkSync(outside, join(directory, "counter.rs"));
    const files = visit(counterRoot(), getRenderMapVisitor());
    throws(() => writeFiles(directory, files));
    equal(readFileSync(outside, "utf8"), "caller bytes");
  } finally {
    rmSync(directory, { recursive: true });
  }
});
