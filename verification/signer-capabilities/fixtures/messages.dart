import 'dart:typed_data';

import 'package:solana_kit_signers/solana_kit_signers.dart';

const hexRadix = 16;

String hex(List<int> bytes) =>
    bytes.map((byte) => byte.toRadixString(hexRadix).padLeft(2, '0')).join();

Future<void> main() async {
  final signer = createKeyPairSignerFromPrivateKeyBytes(
    Uint8List.fromList(List<int>.filled(32, 7)),
  );
  final messages = [
    createSignableMessage(''),
    createSignableMessage('é🦀\u0000'),
    createSignableMessage(Uint8List.fromList([0, 255, 128])),
    createSignableMessage('exact bare bytes'),
  ];
  final outputs = await signer.signMessages(messages);
  print(signer.address.value);

  for (var index = 0; index < messages.length; index++) {
    print(
      '${hex(messages[index].content)}\t'
      '${hex(outputs[index][signer.address]!.value)}',
    );
  }

  final signed = createSignableMessage(
    messages.last.content,
    outputs.last,
  );
  print('${hex(signed.content)}\t${signed.signatures.length}');
}
