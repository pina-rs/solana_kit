import 'dart:typed_data';

import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_keys/solana_kit_keys.dart';
import 'package:solana_kit_offchain_messages/solana_kit_offchain_messages.dart';

const seedBytes = 32;
const firstSeedByte = 7;
const secondSeedByte = 9;
const hexRadix = 16;
const domain = Address('testdomain111111111111111111111111111111111');

String hex(List<int> bytes) =>
    bytes.map((byte) => byte.toRadixString(hexRadix).padLeft(2, '0')).join();

OffchainMessageV0 versionZero(
  OffchainMessageContentFormat format,
  String text,
  List<OffchainMessageSignatory> signers,
) => OffchainMessageV0(
  applicationDomain: domain,
  content: OffchainMessageContent(format: format, text: text),
  requiredSignatories: signers,
);

List<(String, OffchainMessage)> messages(
  List<OffchainMessageSignatory> signers,
) {
  return [
    (
      'v0-ascii',
      versionZero(
        OffchainMessageContentFormat.restrictedAscii1232BytesMax,
        'Hello world',
        signers,
      ),
    ),
    (
      'v0-utf8',
      versionZero(
        OffchainMessageContentFormat.utf81232BytesMax,
        'hi\u0000é',
        signers,
      ),
    ),
    (
      'v0-long',
      versionZero(
        OffchainMessageContentFormat.utf865535BytesMax,
        'こんにちは',
        signers,
      ),
    ),
    (
      'v1',
      OffchainMessageV1(
        content: 'Authorize é\u0000',
        requiredSignatories: signers,
      ),
    ),
  ];
}

void main() {
  final first = createKeyPairFromPrivateKeyBytes(
    Uint8List.fromList(List.filled(seedBytes, firstSeedByte)),
  );
  final second = createKeyPairFromPrivateKeyBytes(
    Uint8List.fromList(List.filled(seedBytes, secondSeedByte)),
  );
  final signers = [second, first]
      .map(
        (key) => OffchainMessageSignatory(
          address: getAddressFromPublicKey(key.publicKey),
        ),
      )
      .toList();

  for (final entry in messages(signers)) {
    final unsigned = compileOffchainMessageEnvelope(entry.$2);
    final signed = signOffchainMessageEnvelope([first, second], unsigned);
    verifyOffchainMessageEnvelope(signed);
    final encoder = getOffchainMessageEnvelopeEncoder();

    print(
      [
        entry.$1,
        hex(unsigned.content),
        hex(encoder.encode(unsigned)),
        hex(encoder.encode(signed)),
      ].join('\t'),
    );
  }

  first.dispose();
  second.dispose();
}
