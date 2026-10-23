import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_codecs_strings/solana_kit_codecs_strings.dart';
import 'package:solana_kit_keys/solana_kit_keys.dart';
import 'package:solana_kit_transaction_introspection/solana_kit_transaction_introspection.dart';
import 'package:solana_kit_transaction_messages/solana_kit_transaction_messages.dart';
import 'package:solana_kit_transactions/solana_kit_transactions.dart';

const addressLength = 32;
const signatureLength = 64;
const signatureByte = 7;
const hexadecimalRadix = 16;

Address key(int byte) => Address(
  getBase58Decoder().decode(Uint8List.fromList(List.filled(addressLength, byte))),
);

CompiledTransactionMessage message(TransactionVersion version) {
  final data = Uint8List.fromList([0, 1, 255]);
  final versionOne = version == TransactionVersion.v1;
  return CompiledTransactionMessage(
    version: version,
    header: const MessageHeader(
      numSignerAccounts: 1,
      numReadonlySignerAccounts: 0,
      numReadonlyNonSignerAccounts: 1,
    ),
    staticAccounts: [key(1), key(4)],
    lifetimeToken: key(9).value,
    instructions: versionOne ? const [] : [
      CompiledInstruction(programAddressIndex: 1, accountIndices: [0, 0], data: data),
    ],
    addressTableLookups: version == TransactionVersion.v0 ? const [] : null,
    configMask: versionOne ? 0 : null,
    configValues: versionOne ? const [] : null,
    numInstructions: versionOne ? 1 : null,
    numStaticAccounts: versionOne ? 2 : null,
    instructionHeaders: versionOne ? const [
      V1InstructionHeader(
        programAccountIndex: 1,
        numInstructionAccounts: 2,
        numInstructionDataBytes: 3,
      ),
    ] : null,
    instructionPayloads: versionOne ? [
      V1InstructionPayload(instructionAccountIndices: [0, 0], instructionData: data),
    ] : null,
  );
}

String hex(Uint8List bytes) => bytes.map((byte) => byte.toRadixString(hexadecimalRadix).padLeft(2, '0')).join();

void main() {
  for (final version in TransactionVersion.values) {
    final compiled = message(version);
    final transaction = Transaction(
      messageBytes: getCompiledTransactionMessageEncoder().encode(compiled),
      signatures: {key(1): SignatureBytes(Uint8List.fromList(List.filled(signatureLength, signatureByte)))},
    );
    final wire = getTransactionEncoder().encode(transaction);
    print('wire\t${version.name}\t${hex(wire)}');
    final instructions = getInstructionsFromCompiledTransactionMessage(compiled);
    print('data\t${version.name}\t${hex(instructions.single.data!)}');
    final traced = walkInstructions(compiledMessage: compiled, meta: {
      'innerInstructions': [
        {'index': 0, 'instructions': [
          {'programIdIndex': 1, 'accounts': [0, 0], 'data': '2', 'stackHeight': 3},
        ]},
      ],
    });

    final inner = traced.last.trace as InnerInstructionTrace;
    print('trace\t${version.name}\t${inner.outerIndex},${inner.innerIndex},${inner.stackHeight}');
  }
}
