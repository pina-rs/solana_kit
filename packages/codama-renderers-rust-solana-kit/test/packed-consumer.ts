import { visit } from "@codama/visitors-core";
import { createFromRoot } from "codama";
import renderVisitor, { getRenderMapVisitor } from "codama-renderers-rust-solana-kit";
import { deepStrictEqual, equal, throws } from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { contractRoot } from "./contracts.ts";

const root = contractRoot();
const first = visit(root, getRenderMapVisitor());
deepStrictEqual([...first], [...visit(root, getRenderMapVisitor())]);
deepStrictEqual([...first], [...createFromRoot(root).accept(getRenderMapVisitor())]);

mkdirSync("generated");
writeFileSync("generated/caller.txt", "keep");
createFromRoot(root).accept(renderVisitor("generated"));
createFromRoot(root).accept(renderVisitor("generated"));

equal(readFileSync("generated/caller.txt", "utf8"), "keep");
equal(readFileSync("generated/counter.rs", "utf8"), first.get("counter.rs")?.content);

const originals = new Map<string, Buffer>();

for (const path of first.keys()) {
  originals.set(path, readFileSync(`generated/${path}`));
}

const invalid = { ...root, program: { ...root.program, publicKey: "notBase58" } };
throws(() => createFromRoot(invalid).accept(renderVisitor("generated")), /Rust renderer rejected/);

for (const [path, original] of originals) {
  deepStrictEqual(readFileSync(`generated/${path}`), original);
}

writeFileSync("root.json", JSON.stringify(root));
