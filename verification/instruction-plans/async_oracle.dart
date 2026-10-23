import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_instruction_plans/solana_kit_instruction_plans.dart';
import 'package:solana_kit_instructions/solana_kit_instructions.dart';
import 'package:solana_kit_transaction_messages/solana_kit_transaction_messages.dart';
import 'package:solana_kit_transactions/solana_kit_transactions.dart';

Future<void> main() async {
  var creates = 0;
  var updates = 0;
  final trace = <String>[];
  final planner = createTransactionPlanner(
    TransactionPlannerConfig(
      maxInstructionsPerTransaction: 1,
      createTransactionMessage: () async {
        await Future<void>.value();
        trace.add('create:${++creates}');
        return TransactionMessage(
          version: TransactionVersion.v0,
          feePayer: const Address(
            '4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi',
          ),
          lifetimeConstraint: BlockhashLifetimeConstraint(
            blockhash: '11111111111111111111111111111111',
            lastValidBlockHeight: BigInt.one,
          ),
        );
      },
      onTransactionMessageUpdated: (message) async {
        await Future<void>.value();
        trace.add('update:${++updates}:${message.instructions.length}');
        return message;
      },
    ),
  );
  final result = await planner(
    sequentialInstructionPlan([
      for (final data in [1, 2])
        Instruction(
          programAddress: const Address('11111111111111111111111111111111'),
          data: Uint8List.fromList([data]),
        ),
    ]),
  );
  print(trace.join(','));

  for (final leaf in flattenTransactionPlan(result)) {
    print(base64Encode(compileTransaction(leaf.message).messageBytes));
  }
}
