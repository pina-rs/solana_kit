// Independent event oracle against the immutable Dart package map.
import 'dart:async';

import 'package:fake_async/fake_async.dart';
import 'package:solana_kit_rpc_types/solana_kit_rpc_types.dart';
import 'package:solana_kit_transaction_confirmation/solana_kit_transaction_confirmation.dart';

const lastValidHeight = 41;
const expiredHeight = 42;

Future<void> flush() => Future<void>.delayed(Duration.zero);
Future<void> main() async {
  for (final a in Commitment.values) {
    for (final b in Commitment.values) {
      print(
        'commitment\t${a.name}\t${b.name}\t${commitmentComparator(a, b).sign}',
      );
    }
  }

  await receipt();
  await expiry();

  for (final level in Commitment.values) {
    fakeAsync((async) {
      var result = 'pending';
      final cancel = CancellationTokenSource();
      unawaited(
        getTimeoutPromise(
          abortSignal: cancel.token,
          commitment: level,
        ).then<void>(
          (_) {},
          onError: (Object error) {
            result = error is TimeoutException ? 'timeout' : 'error';
          },
        ),
      );
      final limit = level == Commitment.processed ? 30000 : 60000;
      async.elapse(Duration(milliseconds: limit - 1));
      final before = result;
      async.elapse(const Duration(milliseconds: 1));
      print('timeout\t${level.name}\t$before>$result\t$limit');
      cancel.cancel();
    });
  }
}

Future<void> receipt() async {
  late void Function({required Object? err, required bool received}) notify;
  final source = CancellationTokenSource();
  var result = 'pending';
  final strategy = createRecentSignatureConfirmationPromiseFactory(
    RecentSignatureConfirmationConfig(
      getSignatureStatuses: (signatures, {required abortSignal}) async => [
        null,
      ],
      onSignatureNotification:
          (
            signature, {
            required abortSignal,
            required commitment,
            required onNotification,
          }) {
            notify = onNotification;
            return Completer<void>().future;
          },
    ),
  );
  final future =
      strategy(
        abortSignal: source.token,
        commitment: Commitment.confirmed,
        signature: 'signature',
      ).then<void>((_) {
        result = 'confirmed';
      });
  await flush();
  notify(err: null, received: true);
  await flush();
  final before = result;
  notify(err: null, received: false);
  await future.timeout(const Duration(seconds: 1));
  print('receipt\t$before>$result');
  source.cancel();
}

Future<void> expiry() async {
  late void Function(SlotNotification notification) notify;
  var height = BigInt.from(lastValidHeight);
  final source = CancellationTokenSource();
  var result = 'pending';
  final strategy = createBlockHeightExceedencePromiseFactory(
    BlockHeightExceedenceConfig(
      getEpochInfo: ({required abortSignal, commitment}) async =>
          EpochInfo(absoluteSlot: height, blockHeight: height),
      onSlotNotification: ({required abortSignal, required onNotification}) {
        notify = onNotification;
        return Completer<void>().future;
      },
    ),
  );
  final future =
      strategy(
        abortSignal: source.token,
        lastValidBlockHeight: BigInt.from(41),
      ).then<void>(
        (_) {},
        onError: (Object error) {
          result = 'expired';
        },
      );
  await flush();
  final before = result;
  height = BigInt.from(expiredHeight);
  notify(SlotNotification(slot: height));
  await future.timeout(const Duration(seconds: 1));
  print('expiry\t41:$before>42:$result');
  source.cancel();
}
