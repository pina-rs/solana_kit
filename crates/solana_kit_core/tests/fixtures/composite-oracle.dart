import 'dart:typed_data';

import 'package:solana_kit_codecs_core/solana_kit_codecs_core.dart';
import 'package:solana_kit_codecs_data_structures/solana_kit_codecs_data_structures.dart';
import 'package:solana_kit_codecs_numbers/solana_kit_codecs_numbers.dart';

const byteValues = 256;
const sourceBits = 27;
const bitsPerByte = 8;
const maximumBitBytes = 4;

String wire(Uint8List bytes) => bytes.isEmpty ? '-' : bytes.join(',');

void emitBits(int size, bool backward, int number) {
  final bits = List.generate(
    sourceBits,
    (index) => ((number >> (index % bitsPerByte)) & 1) != 0,
  );
  final codec = getBitArrayCodec(size, backward: backward);
  final bytes = codec.encode(bits);
  final direction = backward ? 1 : 0;
  final decoded = codec.decode(bytes).map((bit) => bit ? 1 : 0).join();
  final rendered = decoded.isEmpty ? '-' : decoded;
  print('bits\t$size\t$direction\t$number\t${wire(bytes)}\t$rendered');
}

void emitScalars(int number) {
  final base = getU16Codec();
  final padded = padRightCodec(padLeftCodec(base, 2), 3);
  final mapped = transformCodec<num, String, num, String>(
    base,
    (text) => int.parse(text),
    (value, bytes, offset) => value.toString(),
  );
  final sentinel = addCodecSentinel(base, Uint8List.fromList([254, 255]));
  final counted = getArrayCodec(
    getU8Codec(),
    size: PrefixedArraySize(getU8Codec()),
  );
  final values = List.generate(number % 9, (index) => (number + index) & 255);
  final literal = getLiteralUnionCodec([
    'a',
    'b',
    'c',
    'a',
  ], size: getU16Codec());
  final selected = ['a', 'b', 'c'][number % 3];

  print('pad\t$number\t${wire(padded.encode(number))}');
  print('map\t$number\t${wire(mapped.encode('$number'))}');
  print('sentinel\t$number\t${wire(sentinel.encode(number))}');
  print('count\t$number\t${wire(counted.encode(values))}');
  print('literal\t$number\t${wire(literal.encode(selected))}');
}

void emitBitSize(int size) {
  for (final backward in [false, true]) {
    for (var number = 0; number < byteValues; number++) {
      emitBits(size, backward, number);
    }
  }
}

void main() {
  for (var size = 0; size <= maximumBitBytes; size++) {
    emitBitSize(size);
  }

  for (var number = 0; number < byteValues; number++) {
    emitScalars(number);
  }
}
