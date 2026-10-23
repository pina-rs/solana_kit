import 'dart:typed_data';

import 'package:solana_kit_accounts/solana_kit_accounts.dart';
import 'package:solana_kit_addresses/solana_kit_addresses.dart';
import 'package:solana_kit_errors/solana_kit_errors.dart';

const owner = '11111111111111111111111111111111';
const address = Address(owner);
const dataValue = 7;
Map<String, Object?> header() => {
  'executable': false,
  'lamports': 1,
  'owner': owner,
  'space': 3,
};

void parsedMetadata() {
  final raw = <String, Object?>{
    ...header(),
    'data': <String, Object?>{
      'parsed': <String, Object?>{
        'info': <String, Object?>{'x': dataValue},
        'type': 'custom',
      },
    },
  };
  final parsed = parseJsonRpcAccount(
    address,
    raw,
  ) as ExistingAccount<JsonParsedAccountData<Map<String, Object?>>>;
  assert(parsed.data.data['x'] == dataValue);
  assert(parsed.data.parsedAccountMeta!.program == '');
  assert(parsed.data.parsedAccountMeta!.type == 'custom');
  const typed = JsonParsedAccountData<int>(data: dataValue);
  assert(typed.data == dataValue && typed.parsedAccountMeta == null);
  print('optional parsed metadata and typed scalar construction execute');
}

void stringMetadata() {
  final raw = <String, Object?>{...header(), 'lamports': '1', 'space': '3'};
  final base = parseBaseAccount(raw);
  assert(base.lamports.value == BigInt.one);
  assert(base.space == BigInt.from(3));
  final strings = parseBase64RpcAccount(address, {
    ...raw,
    'data': ['AQID', 'base64'],
  }) as ExistingAccount<Uint8List>;
  assert(strings.lamports.value == BigInt.one);
  assert(strings.space == BigInt.from(3));
  print('metadata-only and byte account string numeric metadata execute');
}

void assertionAddress() {
  final encoded = parseBase64RpcAccount(address, {
    ...header(),
    'data': ['', 'base64'],
  }) as ExistingAccount<Uint8List>;

  try {
    assertAccountDecoded(encoded.account);
    throw StateError('expected decoded assertion failure');
  } on SolanaError catch (error) {
    assert(error.context['address'] == owner);
    print('decoded assertion preserves address ${error.context['address']}');
  }
}

void main() {
  parsedMetadata();
  stringMetadata();
  assertionAddress();
}
