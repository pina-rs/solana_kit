import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_errors/solana_kit_errors.dart';
import 'package:solana_kit_keys/solana_kit_keys.dart';
import 'package:solana_kit_signers/solana_kit_signers.dart';
import 'package:solana_kit_transactions/solana_kit_transactions.dart';

const signatureBytes = 64;
const capabilityLimit = 8;
const partialSending = 5;
const modifyingSending = 6;
const composite = 7;

final events = <String>[];

class Base {
  Base(this.id);
  final int id;
  Address get address => Address(
    id == 0
        ? '22222222222222222222222222222222222222222222'
        : '33333333333333333333333333333333333333333333',
  );
  Future<List<Map<Address, SignatureBytes>>> signTransactions(
    List<Transaction> txs, [
    TransactionSignerConfig? config,
  ]) async {
    events.add('$id:p');

    return txs.map((_) => <Address, SignatureBytes>{}).toList();
  }

  Future<List<Transaction>> modifyAndSignTransactions(
    List<Transaction> txs, [
    TransactionSignerConfig? config,
  ]) async {
    events.add('$id:m');

    return txs;
  }

  Future<List<SignatureBytes>> signAndSendTransactions(
    List<Transaction> txs, [
    TransactionSignerConfig? config,
  ]) async {
    events.add('$id:s');

    return txs
        .map(
          (_) => SignatureBytes(
            Uint8List.fromList(List.filled(signatureBytes, 1)),
          ),
        )
        .toList();
  }
}

class C1 extends Base implements TransactionPartialSigner {
  C1(super.id);
}

class C2 extends Base implements TransactionModifyingSigner {
  C2(super.id);
}

class C3 extends Base
    implements TransactionPartialSigner, TransactionModifyingSigner {
  C3(super.id);
}

class C4 extends Base implements TransactionSendingSigner {
  C4(super.id);
}

class C5 extends Base
    implements TransactionPartialSigner, TransactionSendingSigner {
  C5(super.id);
}

class C6 extends Base
    implements TransactionModifyingSigner, TransactionSendingSigner {
  C6(super.id);
}

class C7 extends Base
    implements
        TransactionPartialSigner,
        TransactionModifyingSigner,
        TransactionSendingSigner {
  C7(super.id);
}

Base make(int flags, int id) => switch (flags) {
  1 => C1(id),
  2 => C2(id),
  3 => C3(id),
  4 => C4(id),
  partialSending => C5(id),
  modifyingSending => C6(id),
  composite => C7(id),
  _ => throw StateError('bad flags'),
};

Future<void> run(List<int> flags, bool sending) async {
  events.clear();

  final signers = [for (var i = 0; i < flags.length; i++) make(flags[i], i)];
  final tx = Transaction(messageBytes: Uint8List.fromList([1]), signatures: {});
  var result = '';
  try {
    if (sending) {
      await signAndSendTransactionWithSigners(signers, tx);
    } else {
      await partiallySignTransactionWithSigners(signers, tx);
    }

    result = [
      for (var i = 0; i < flags.length; i++)
        events
                .where((e) => e.startsWith('$i:'))
                .map((e) => e.substring(2))
                .firstOrNull ??
            'i',
    ].join(',');
  } on SolanaError catch (error) {
    result = switch (error.code) {
      SolanaErrorCode.signerTransactionSendingSignerMissing => 'missing',
      SolanaErrorCode.signerTransactionCannotHaveMultipleSendingSigners =>
        'multiple',
      _ => throw error,
    };
  }
  print('${flags.join(",")}\t${sending ? 1 : 0}\t$result');
}

Future<void> main() async {
  for (var a = 1; a < capabilityLimit; a++) {
    for (final send in [false, true]) {
      await run([a], send);
    }
  }

  for (var a = 1; a < capabilityLimit; a++) {
    for (var b = 1; b < capabilityLimit; b++) {
      for (final send in [false, true]) {
        await run([a, b], send);
      }
    }
  }
}
