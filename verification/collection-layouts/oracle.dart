import 'dart:typed_data';

import 'package:solana_kit_codecs_data_structures/solana_kit_codecs_data_structures.dart';
import 'package:solana_kit_codecs_numbers/solana_kit_codecs_numbers.dart';
import 'package:solana_kit_options/solana_kit_options.dart';

String wire(Uint8List bytes) => bytes.isEmpty ? '-' : bytes.join(',');

void arrays() {
  for (final strategy in SentinelCountStrategy.values) {
    final codec = getArrayCodec(
      getU16Codec(),
      size: SentinelArraySize(Uint8List.fromList([255]), strategy: strategy),
    );

    for (final values in <List<num>>[
      [],
      [65281],
      [65281, 513],
    ]) {
      print(
        'array\t${strategy.index}\t${values.join(',')}\t${wire(codec.encode(values))}',
      );
    }
  }
}

void optional(bool tagged, int kind) {
  final nullable = getNullableCodec(
    getU16Codec(),
    hasPrefix: tagged,
    noneValue: switch (kind) {
      0 => const OmitNoneValue(),
      1 => const ZeroesNoneValue(),
      _ => ConstantNoneValue(Uint8List.fromList([170, 187])),
    },
  );
  final option = getOptionCodec(
    getU16Codec(),
    hasPrefix: tagged,
    noneValue: switch (kind) {
      0 => const OmitOptionNoneValue(),
      1 => const ZeroesOptionNoneValue(),
      _ => ConstantOptionNoneValue(Uint8List.fromList([170, 187])),
    },
  );

  for (final value in <num?>[null, 513]) {
    final label = value == null ? 'none' : '$value';
    final config = '${tagged ? 1 : 0},$kind';
    print('nullable\t$config\t$label\t${wire(nullable.encode(value))}');
    print('option\t$config\t$label\t${wire(option.encode(value))}');
  }
}

void booleans() {
  for (final value in [false, true]) {
    final codec = getBooleanCodec(size: getU32Codec());
    print('boolean\t0\t${value ? 1 : 0}\t${wire(codec.encode(value))}');
  }
}

void main() {
  arrays();

  for (final tagged in [true, false]) {
    for (var kind = 0; kind < 3; kind++) {
      optional(tagged, kind);
    }
  }

  booleans();
}
