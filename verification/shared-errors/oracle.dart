import 'dart:convert';

import 'package:solana_kit_errors/solana_kit_errors.dart';
const instructionIndex = 7;

void emit(String input, SolanaError error) {
  final facts = error.context.map(
    (key, value) => MapEntry(key, value.toString()),
  );
  print('$input\t${error.code.value}\t${jsonEncode(facts)}');
}

void main() {
  for (final code in SolanaErrorCode.values) {
    final value = executionInput(code);

    if (value != null) {
      emit(jsonEncode(value), getSolanaErrorFromTransactionError(value));
    }
  }

  for (final value in <Object>[
    {
      'FutureFailure': {'value': 9223372036854775807},
    },
    {
      'InstructionError': [
        255,
        {'FutureInstruction': 4294967295},
      ],
    },
    {
      'InstructionError': [
        3,
        {'BorshIoError': 'kept in typed source'},
      ],
    },
  ]) {
    emit(jsonEncode(value), getSolanaErrorFromTransactionError(value));
  }
}

Object? executionInput(SolanaErrorCode code) {
  final instruction = code.name.startsWith('instructionError');
  final transaction = code.name.startsWith('transactionError');

  if ((!instruction && !transaction) || code.name.endsWith('Unknown')) {
    return null;
  }

  final prefix = instruction ? 'instructionError' : 'transactionError';
  final name = code.name.substring(prefix.length);
  final value = payload(name);

  return instruction
      ? {
          'InstructionError': [instructionIndex, value],
        }
      : value;
}

Object payload(String name) {
  return switch (name) {
    'Custom' => {name: 4294967295},
    'DuplicateInstruction' => {name: 255},
    'InsufficientFundsForRent' || 'ProgramExecutionTemporarilyRestricted' => {
      name: {'account_index': 255},
    },
    _ => name,
  };
}
