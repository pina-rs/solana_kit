import 'dart:typed_data';

import 'package:solana_kit_signers/solana_kit_signers.dart';

const hexRadix = 16;

Future<void> main() async {
  final signer = createKeyPairSignerFromPrivateKeyBytes(
    Uint8List.fromList(List<int>.filled(32, 7)),
  );
  final message = createSignableMessage('exact bare bytes');
  final results = await signer.signMessages([message]);
  final signature = results.single[signer.address]!;
  print(signer.address.value);
  print(
    signature.value
        .map((byte) => byte.toRadixString(hexRadix).padLeft(2, '0'))
        .join(),
  );
}
