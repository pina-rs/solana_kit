import 'dart:convert';

import 'package:solana_kit_errors/solana_kit_errors.dart';

const oracleSlot = 55;

void main() {
  final cause = SolanaError(SolanaErrorCode.blockHeightExceeded, {
    'slot': oracleSlot,
  });
  final context = createSolanaErrorContext({
    'operation': 'testOperation',
    'path': null,
  }, cause: cause);
  print(
    jsonEncode({
      'operation': context['operation'],
      'hasPath': context.containsKey('path'),
      'causeCode': (context['causeCode']! as SolanaErrorCode).value,
      'causeContext': context['causeContext'],
      'causeType': context['causeType'],
    }),
  );
}
