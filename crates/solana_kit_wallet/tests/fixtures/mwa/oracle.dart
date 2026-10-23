import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:pointycastle/export.dart';
import 'package:solana_kit_mobile_wallet_adapter_protocol/solana_kit_mobile_wallet_adapter_protocol.dart';

const hexRadix = 16;
const pointBytes = 65;
String hex(List<int> bytes) =>
    bytes.map((byte) => byte.toRadixString(hexRadix).padLeft(2, '0')).join();
Uint8List unhex(String value) => Uint8List.fromList([
  for (var index = 0; index < value.length; index += 2)
    int.parse(value.substring(index, index + 2), radix: hexRadix),
]);

EcdhKeypair agreement(int scalar) {
  final secret = BigInt.from(scalar);
  return EcdhKeypair(
    publicKey: ECPublicKey(p256.G * secret, p256),
    privateKey: ECPrivateKey(secret, p256),
  );
}

Uint8List deterministicSignature(Uint8List message, ECPrivateKey key) {
  final signer = ECDSASigner(SHA256Digest(), HMac(SHA256Digest(), 64))
    ..init(true, PrivateKeyParameter<ECPrivateKey>(key));
  final signature = signer.generateSignature(message) as ECSignature;
  final r = signature.r.toRadixString(hexRadix).padLeft(64, '0');
  final s = signature.s.toRadixString(hexRadix).padLeft(64, '0');

  return unhex('$r$s');
}

String deterministicJws(ECPrivateKey key) {
  const header = 'eyJhbGciOiJFUzI1NiJ9';
  const payload = 'eyJ0ZXN0Ijp0cnVlfQ';
  const message = '$header.$payload';
  final signature = deterministicSignature(
    Uint8List.fromList(utf8.encode(message)),
    key,
  );
  final encoded = base64Url.encode(signature).replaceAll('=', '');

  return '$message.$encoded';
}

void main(List<String> arguments) {
  final association = AssociationKeypair(
    publicKey: ECPublicKey(p256.G, p256),
    privateKey: ECPrivateKey(BigInt.one, p256),
  );
  final alice = agreement(2);
  final bob = agreement(3);
  final session = parseHelloRsp(
    exportEcdhPublicKeyBytes(bob),
    association,
    alice,
  ).sharedSecret;

  if (arguments.isNotEmpty) {
    verifyRust(arguments.single, association, session);

    return;
  }

  stdout.writeln(jsonEncode(vectors(association, alice, bob, session)));
}

Map<String, Object?> vectors(
  AssociationKeypair association,
  EcdhKeypair alice,
  EcdhKeypair bob,
  Uint8List session,
) {
  final nonce = Uint8List.fromList(List.generate(12, (index) => index));
  final aad = createSequenceNumberVector(1);
  final encrypted = aesGcmEncrypt(
    plaintext: Uint8List.fromList(utf8.encode('hello wallet')),
    key: session,
    nonce: nonce,
    aad: aad,
  );

  return {
    'association': hex(exportPublicKeyBytes(association.publicKey)),
    'alice': hex(exportEcdhPublicKeyBytes(alice)),
    'bob': hex(exportEcdhPublicKeyBytes(bob)),
    'shared': hex(ecdhSharedSecret(alice.privateKey, bob.publicKey)),
    'key': hex(session),
    'hello': hex([
      ...exportEcdhPublicKeyBytes(alice),
      ...deterministicSignature(
        exportEcdhPublicKeyBytes(alice),
        association.privateKey,
      ),
    ]),
    'frame': hex([...aad, ...nonce, ...encrypted]),
    'jws': deterministicJws(association.privateKey),
    'siws': createSiwsMessage(
      const SignInPayload(
        domain: 'example.com',
        address: '11111111111111111111111111111111',
        nonce: 'abcdefgh',
        resources: ['https://example.com/read'],
      ),
    ),
  };
}

void verifyRust(
  String file,
  AssociationKeypair association,
  Uint8List session,
) {
  final rust =
      jsonDecode(File(file).readAsStringSync()) as Map<String, Object?>;
  final hello = unhex(rust['hello']! as String);

  if (!ecdsaVerify(
    hello.sublist(0, pointBytes),
    hello.sublist(pointBytes),
    association.publicKey,
  )) {
    throw StateError('Rust HELLO signature rejected');
  }

  if (decryptMessage(unhex(rust['frame']! as String), session).plaintext !=
      'hello wallet') {
    throw StateError('Rust frame rejected');
  }

  final jws = (rust['jws']! as String).split('.');

  if (!ecdsaVerify(
    Uint8List.fromList(utf8.encode('${jws[0]}.${jws[1]}')),
    Uint8List.fromList(base64Url.decode(base64Url.normalize(jws[2]))),
    association.publicKey,
  )) {
    throw StateError('Rust ES256 JWS rejected');
  }

  stdout.writeln('Dart accepted Rust HELLO, AES-GCM frame, and ES256 JWS');
}
