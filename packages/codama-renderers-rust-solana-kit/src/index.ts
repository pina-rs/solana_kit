import { rootNodeVisitor, visit } from "@codama/visitors-core";
import type { Visitor } from "@codama/visitors-core";
import { render } from "./engine.js";
import type { RustRenderMap } from "./files.js";
import { writeFiles } from "./write.js";

export type { RustRenderMap } from "./files.js";

/** Produces a standard Codama render map through the shared Rust engine. */
export function getRenderMapVisitor(): Visitor<RustRenderMap, "rootNode"> {
  return rootNodeVisitor(render);
}

/** Repeatedly renders the selected modules while preserving unrelated caller files. */
export function renderVisitor(outputDirectory: string): Visitor<void, "rootNode"> {
  return rootNodeVisitor((root): void => {
    const files = visit(root, getRenderMapVisitor());
    writeFiles(outputDirectory, files);
  });
}

export default renderVisitor;
