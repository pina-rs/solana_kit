import 'package:solana_kit_errors/solana_kit_errors.dart';

const oracleCount = 42;

void main() {
  for (final value in <Map<String, Object?>>[
    {},
    {'count': oracleCount, 'key': null},
    {
      'items': ['a', 'b'],
    },
    {
      'nested': {'x': 1},
    },
    {
      'items': ['a b', '%'],
    },
  ]) {
    print(encodeContextObject(value));
  }
}
