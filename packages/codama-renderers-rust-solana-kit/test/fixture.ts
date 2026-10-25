import {
  fieldDiscriminatorNode,
  instructionArgumentNode,
  instructionNode,
  numberTypeNode,
  numberValueNode,
  programNode,
  rootNode,
} from "@codama/nodes";

/** Uses real pinned Codama factories, including their omitted empty collections. */
export function counterRoot() {
  return rootNode(programNode({
    name: "counter",
    publicKey: "11111111111111111111111111111111",
    instructions: [instructionNode({
      name: "setValue",
      arguments: [
        instructionArgumentNode({
          name: "discriminator",
          type: numberTypeNode("u8"),
          defaultValue: numberValueNode(7),
          defaultValueStrategy: "omitted",
        }),
        instructionArgumentNode({ name: "value", type: numberTypeNode("u64") }),
      ],
      discriminators: [fieldDiscriminatorNode("discriminator")],
    })],
  }));
}
