import 'dart:convert';
import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_instruction_plans/solana_kit_instruction_plans.dart';
import 'package:solana_kit_instructions/solana_kit_instructions.dart';
import 'package:solana_kit_keys/solana_kit_keys.dart';
import 'package:solana_kit_transaction_messages/solana_kit_transaction_messages.dart';
import 'package:solana_kit_transactions/solana_kit_transactions.dart';

const largeInstruction = 800;
const instructionCount = 5;

TransactionMessage draft() => TransactionMessage(
  version: TransactionVersion.v0,
  feePayer: const Address('E9Nykp3rSdza2moQutaJ3K3RSC8E5iFERX2SqLTsQfjJ'),
  lifetimeConstraint: BlockhashLifetimeConstraint(
    blockhash: '11111111111111111111111111111111',
    lastValidBlockHeight: BigInt.one,
  ),
);
Instruction instruction(int length) => Instruction(
  programAddress: const Address('11111111111111111111111111111111'),
  data: Uint8List(length),
);

String shape(TransactionPlan plan) => switch (plan) {
  SingleTransactionPlan(:final message) => '${message.instructions.length}',
  SequentialTransactionPlan(:final plans, :final divisible) =>
    '${divisible ? 'S' : 'A'}(${plans.map(shape).join(',')})',
  ParallelTransactionPlan(:final plans) => 'P(${plans.map(shape).join(',')})',
};

Future<void> main() async {
  final cases = <String, InstructionPlan>{
    'sequential-count': sequentialInstructionPlan(
      List.generate(instructionCount, (_) => instruction(0)),
    ),
    'parallel-count': parallelInstructionPlan(
      List.generate(instructionCount, (_) => instruction(0)),
    ),
    'sequential-bytes': sequentialInstructionPlan([
      instruction(largeInstruction),
      instruction(100),
      instruction(largeInstruction),
    ]),
    'atomic-bytes': nonDivisibleSequentialInstructionPlan([
      instruction(largeInstruction),
      instruction(largeInstruction),
    ]),
    'packed-bytes': sequentialInstructionPlan([
      instruction(largeInstruction),
      getMessagePackerInstructionPlanFromInstructions([
        instruction(100),
        instruction(largeInstruction),
      ]),
    ]),
    'mixed': sequentialInstructionPlan([
      parallelInstructionPlan([
        instruction(largeInstruction),
        instruction(largeInstruction),
      ]),
      instruction(100),
    ]),
  };

  for (final entry in cases.entries) {
    final planner = createTransactionPlanner(
      TransactionPlannerConfig(
        createTransactionMessage: () async => draft(),
        maxInstructionsPerTransaction: 2,
      ),
    );
    final plan = await planner(entry.value);
    print('${entry.key}\t${shape(plan)}');

    for (final leaf in flattenTransactionPlan(plan)) {

      final bytes = compileTransaction(leaf.message).messageBytes;
      print('wire\t${base64Encode(bytes)}');
    }
  }

  final executor = createTransactionPlanExecutor(
    TransactionPlanExecutorConfig(
      executeTransactionMessage: (context, message) async {
        context['progress'] = 'recorded';
        throw StateError('declined');
      },
    ),
  );
  final result = await passthroughFailedTransactionPlanExecution(
    executor(sequentialTransactionPlan([draft(), draft(), draft()])),
  );
  print(
    'failure\t${flattenTransactionPlanResult(result).map((leaf) => leaf.status.name).join(',')}',
  );
  final signature = Signature('1' * 64);
  final concurrent = createTransactionPlanExecutorWithConcurrentLeaves(
    TransactionPlanExecutorConfig(
      executeTransactionMessage: (context, message) async => {
        'signature': signature,
      },
    ),
  );
  final concurrentResult = await concurrent(
    nonDivisibleSequentialTransactionPlan([draft(), draft()]),
  );
  print(
    'concurrent\t${flattenTransactionPlanResult(concurrentResult).map((leaf) => leaf.status.name).join(',')}',
  );
}
