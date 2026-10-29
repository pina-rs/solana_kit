import { createFromJson } from "codama";
import { counterRoot } from "./fixture.ts";
import numeric from "./numeric.json" with { type: "json" };

const PROGRAM = "11111111111111111111111111111111";

function number(format: "u8" | "u64") {
  return { kind: "numberTypeNode", format, endian: "le" };
}

function role(name: string, optional = false, writable = false) {
  return {
    kind: "instructionAccountNode",
    name,
    isOptional: optional,
    isWritable: writable,
    isSigner: false,
  };
}

function instruction(name: string, tag: number) {
  return {
    kind: "instructionNode",
    name,
    arguments: [{
      kind: "instructionArgumentNode",
      name: "discriminator",
      type: number("u8"),
      defaultValue: { kind: "numberValueNode", number: tag },
      defaultValueStrategy: "omitted",
    }],
    discriminators: [{ kind: "fieldDiscriminatorNode", name: "discriminator" }],
  };
}

function pda(name: string, text: string) {
  return {
    kind: "pdaNode",
    name,
    seeds: [{
      kind: "constantPdaSeedNode",
      type: { kind: "stringTypeNode", encoding: "utf8" },
      value: { kind: "stringValueNode", string: text },
    }],
  };
}

function discriminator(hex: string) {
  return [{
    kind: "constantDiscriminatorNode",
    offset: 0,
    constant: {
      kind: "constantValueNode",
      type: { kind: "bytesTypeNode" },
      value: { kind: "bytesValueNode", encoding: "base16", data: hex },
    },
  }];
}

function record(name: string, hex: string) {
  return {
    name,
    data: {
      kind: "structTypeNode",
      fields: [
        { kind: "structFieldTypeNode", name: "magic", type: number("u8") },
        { kind: "structFieldTypeNode", name: "value", type: number("u64") },
      ],
    },
    discriminators: discriminator(hex),
  };
}

function recordProgram() {
  return {
    kind: "programNode",
    name: "recordContracts",
    publicKey: PROGRAM,
    instructions: [instruction("ping", 1)],
    accounts: [{ kind: "accountNode", ...record("stateRecord", "07") }],
    events: [{ kind: "eventNode", ...record("stateChanged", "09") }],
  };
}

function resolverProgram() {
  const derived = {
    ...role("derived"),
    defaultValue: {
      kind: "pdaValueNode",
      pda: { kind: "pdaLinkNode", name: "byProgram" },
      seeds: [{
        kind: "pdaSeedValueNode",
        name: "program",
        value: { kind: "accountValueNode", name: "program" },
      }],
    },
  };

  return {
    kind: "programNode",
    name: "resolverContracts",
    publicKey: PROGRAM,
    pdas: [pda("x", "one"), pda("xWithProgram", "two"), {
      kind: "pdaNode",
      name: "byProgram",
      seeds: [{
        kind: "variablePdaSeedNode",
        name: "program",
        type: { kind: "publicKeyTypeNode" },
      }],
    }],
    instructions: [
      {
        ...instruction("setRoles", 1),
        accounts: [
          role("program"),
          role("maybe", true, true),
          role("accounts", false, true),
          derived,
        ],
      },
      {
        ...instruction("omitRoles", 2),
        optionalAccountStrategy: "omitted",
        accounts: [role("first", true), role("second", true)],
      },
    ],
  };
}

function literalProgram() {
  return {
    kind: "programNode",
    name: "literalContracts",
    publicKey: PROGRAM,
    instructions: [{
      ...instruction("setLiteral", 1),
      accounts: [{
        ...role("systemProgram"),
        defaultValue: { kind: "publicKeyValueNode", publicKey: PROGRAM },
      }],
    }],
  };
}

/** Legal schemas exercise discriminator predicates and every resolver binding boundary. */
export function contractRoot() {
  const emptyPda = {
    ...counterRoot().program,
    name: "emptyPdaContracts",
    pdas: [pda("constant", "empty")],
  };

  return createFromJson(
    JSON.stringify({
      ...counterRoot(),
      additionalPrograms: [recordProgram(), resolverProgram(), literalProgram(), emptyPda, numeric],
    }),
  ).getRoot();
}
