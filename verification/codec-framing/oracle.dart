import 'dart:typed_data';

import 'package:solana_kit_codecs_core/solana_kit_codecs_core.dart';
import 'package:solana_kit_codecs_numbers/solana_kit_codecs_numbers.dart';

const maxWidth = 8;
const maxLength = 9;
const step = 17;

String wire(List<int> bytes) => bytes.isEmpty ? '-' : bytes.join(',');

VariableSizeCodec<Uint8List, Uint8List> rawCodec() => VariableSizeCodec(
  getSizeFromValue: (value) => value.length,
  write: (value, bytes, offset) {
    bytes.setAll(offset, value);
    return offset + value.length;
  },
  read: (bytes, offset) => (bytes.sublist(offset), bytes.length),
);

void frames() {
  final raw = rawCodec();

  for (var width = 0; width <= maxWidth; width++) {
    for (var length = 0; length <= maxLength; length++) {
      final value = Uint8List.fromList(
        List.generate(length, (index) => index * step + 3),
      );
      final fixed = fixCodecSize(raw, width);
      print('fixed\t$width\t$length\t${wire(fixed.encode(value))}');

      final prefixed = addCodecSizePrefix(raw, getU8Codec());
      print('prefix\t$width\t$length\t${wire(prefixed.encode(value))}');

      final resized = resizeCodec(raw, (size) => size + width);
      print('resize\t$width\t$length\t${wire(resized.encode(value))}');
    }
  }
}

void reversedFields() {
  for (var width = 0; width <= maxWidth; width++) {
    final fixed = FixedSizeCodec<Uint8List, Uint8List>(
      fixedSize: width,
      write: (value, bytes, offset) {
        bytes.setAll(offset, value);
        return offset + width;
      },
      read: (bytes, offset) =>
          (bytes.sublist(offset, offset + width), offset + width),
    );
    final value = Uint8List.fromList(
      List.generate(width, (index) => index + 1),
    );
    print(
      'reverse\t$width\t$width\t${wire(reverseCodec(fixed).encode(value))}',
    );
  }
}

void sentinelRow(List<int> payload, List<int> delimiter) {
  try {
    final encoded = addCodecSentinel(
      rawCodec(),
      Uint8List.fromList(delimiter),
    ).encode(Uint8List.fromList(payload));
    print('sentinel\t${wire(payload)}\t${wire(delimiter)}\t${wire(encoded)}');
  } on Object {
    print('sentinel\t${wire(payload)}\t${wire(delimiter)}\terror');
  }
}

void sentinels() {
  for (final payload in <List<int>>[
    [],
    [1],
    [1, 1],
    [2, 1],
    [2, 2],
  ]) {
    for (final delimiter in <List<int>>[
      [],
      [1],
      [1, 1],
      [1, 2],
    ]) {
      sentinelRow(payload, delimiter);
    }
  }
}

void main() {
  frames();
  reversedFields();
  sentinels();
}
