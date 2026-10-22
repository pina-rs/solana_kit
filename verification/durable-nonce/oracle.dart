import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_codecs_strings/solana_kit_codecs_strings.dart';
import 'package:solana_kit_instructions/solana_kit_instructions.dart';
import 'package:solana_kit_transaction_messages/solana_kit_transaction_messages.dart';

const hashLength = 32;
const prefixScenario = 5;
const sharedPayerScenario = 6;
const duplicateRoleScenario = 7;
const hexRadix = 16;
const changedAuthorityByte = 5;
const prefixByte = 42;
const maximumScenario = 9;
const minimumScenario = 8;
const ordinaryNonceByte = 7;

String token(int byte) => getBase58Decoder().decode(
  Uint8List.fromList(List.filled(hashLength, byte)),
);
Address address(int byte) => Address(token(byte));
String hex(List<int> bytes) =>
    bytes.map((byte) => byte.toRadixString(hexRadix).padLeft(2, '0')).join();

String instructionText(Instruction instruction) {
  final accounts = instruction.accounts
      ?.map((meta) => '${meta.address.value}:${meta.role.value}')
      .join(',');
  final data = hex(instruction.data ?? []);

  return '${instruction.programAddress.value},${accounts ?? ''},$data';
}

Instruction? firstInstruction(int scenario, Address authority) {
  return switch (scenario) {
    1 || 2 => createAdvanceNonceAccountInstruction(address(2), authority),
    3 => createAdvanceNonceAccountInstruction(address(4), address(3)),
    4 => createAdvanceNonceAccountInstruction(
      address(2),
      address(changedAuthorityByte),
    ),
    prefixScenario => Instruction(
      programAddress: address(9),
      data: Uint8List.fromList([prefixByte]),
    ),
    _ => null,
  };
}

void emit(TransactionVersion version, int scenario) {
  final nonceByte = scenario == minimumScenario
      ? 0
      : scenario == maximumScenario
      ? 255
      : ordinaryNonceByte;
  final authority = address(switch (scenario) {
    sharedPayerScenario => 1,
    duplicateRoleScenario => 2,
    _ => 3,
  });
  final config = DurableNonceConfig(
    nonce: token(nonceByte),
    nonceAccountAddress: address(2),
    nonceAuthorityAddress: authority,
  );
  final first = firstInstruction(scenario, authority);
  final input = TransactionMessage(
    version: version,
    feePayer: address(1),
    instructions: first == null ? [] : [first],
    lifetimeConstraint: scenario == 2
        ? DurableNonceLifetimeConstraint(nonce: token(minimumScenario))
        : null,
  );
  final result = setTransactionMessageLifetimeUsingDurableNonce(config, input);
  final repeated = setTransactionMessageLifetimeUsingDurableNonce(
    config,
    result,
  );

  if (!identical(result, repeated)) throw StateError('not idempotent');

  final compiled = compileTransactionMessage(result);
  final bytes = getCompiledTransactionMessageEncoder().encode(compiled);
  final metas = result.instructions.map(instructionText).join(';');
  final decompiled = decompileTransactionMessage(compiled);
  final lifetime = result.lifetimeConstraint as DurableNonceLifetimeConstraint;
  print(
    [
      version.name,
      scenario,
      config.nonce,
      lifetime.nonce,
      metas,
      hex(bytes),
      decompiled.lifetimeConstraint.runtimeType,
    ].join('\t'),
  );
}

void main() {
  for (final version in TransactionVersion.values) {
    for (var scenario = 0; scenario < 10; scenario++) {
      emit(version, scenario);
    }
  }
}
