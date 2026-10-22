import 'dart:typed_data';

import 'package:solana_kit_offchain_messages/solana_kit_offchain_messages.dart';

import 'oracle.dart' show domain, versionZero;

const shortMaximum = 1232;
const longMaximum = 65535;
const versionOffset = 16;
const formatOffset = 49;
const signatoryCountOffset = 50;
const v0LengthOffset = 83;
const v1PreambleBytes = 50;

void check(String name, void Function() operation, bool accepted) {
  var succeeded = true;

  try {
    operation();
  } catch (_) {
    succeeded = false;
  }

  if (succeeded != accepted) {
    throw StateError('$name acceptance differs from expected $accepted');
  }

  print('$name\t${succeeded ? "accepted" : "rejected"}');
}

void versionZeroBoundaries(List<OffchainMessageSignatory> signers) {
  for (final format in OffchainMessageContentFormat.values) {
    final maximum = format == OffchainMessageContentFormat.utf865535BytesMax
        ? longMaximum
        : shortMaximum;

    for (final size in [0, maximum, maximum + 1]) {
      check('v0-${format.value}-$size', () {
        final message = versionZero(format, 'a' * size, signers);
        final wire = getOffchainMessageV0Encoder().encode(message);
        getOffchainMessageV0Decoder().decode(wire);
      }, size > 0 && size <= maximum);
    }
  }
}

void malformedVersionZero(List<OffchainMessageSignatory> signers) {
  final original = getOffchainMessageV0Encoder().encode(
    versionZero(
      OffchainMessageContentFormat.utf81232BytesMax,
      'hello',
      signers,
    ),
  );

  for (final offset in [
    1,
    versionOffset,
    formatOffset,
    signatoryCountOffset,
    v0LengthOffset,
  ]) {
    check('invalid-v0-$offset', () {
      final bytes = Uint8List.fromList(original);
      bytes[offset] = 255;
      getOffchainMessageDecoder().decode(bytes);
    }, false);
  }
}

void versionOneBoundaries(List<OffchainMessageSignatory> signers) {
  final versionOne = getOffchainMessageV1Encoder().encode(
    OffchainMessageV1(
      content: 'a' * (longMaximum + 1),
      requiredSignatories: signers,
    ),
  );
  check('v1-65536', () => getOffchainMessageDecoder().decode(versionOne), true);
  check('empty-v1', () {
    getOffchainMessageDecoder().decode(
      Uint8List.fromList(versionOne.sublist(0, v1PreambleBytes)),
    );
  }, false);
  check('invalid-utf8', () {
    getOffchainMessageDecoder().decode(
      Uint8List.fromList([...versionOne, 255]),
    );
  }, false);
  check('v0-bom-source-decoder-divergence', () {
    final wire = getOffchainMessageV0Encoder().encode(
      versionZero(
        OffchainMessageContentFormat.utf81232BytesMax,
        '\uFEFFhi',
        signers,
      ),
    );
    getOffchainMessageDecoder().decode(wire);
  }, false);
}

void main() {
  final signers = [OffchainMessageSignatory(address: domain)];
  versionZeroBoundaries(signers);
  malformedVersionZero(signers);
  versionOneBoundaries(signers);
}
