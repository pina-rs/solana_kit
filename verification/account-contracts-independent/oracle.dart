import 'dart:convert';

import 'package:solana_kit_accounts/solana_kit_accounts.dart';

const owner = '11111111111111111111111111111111';
const unicodeLimit = 0x110000;
const sample = 7;

BigInt parse(Object? value) => parseBaseAccount({
  'executable': true,
  'lamports': value,
  'owner': owner,
  'space': value,
}).space;

/// Prints independent grammar results and the complete accepted scalar set.
void main() {
  // Every scalar is observed through the pinned parser; no Unicode table is assumed.
  for (final value in [
    '0',
    '+0',
    '00',
    '0007',
    '+0007',
    '0x0',
    '0XfF',
    '+0XfF',
    '  +0x2a\t',
    '-0',
    '-0x0',
    '-1',
    '1.0',
    '1e0',
    '1E2',
    '0b1',
    '0o7',
    '0x',
    '0x+1',
    '0x-1',
    '++1',
    '+ 1',
    '1_0',
    '',
    ' ',
    'ff',
    '0xffg',
    '１２',
    '١',
    '18446744073709551615',
    '18446744073709551616',
    '0xffffffffffffffff',
    '0x10000000000000000',
    true,
    null,
    1.5,
  ]) {
    print('${jsonEncode(value)}\t${result(value)}');
  }

  final accepted = <int>[];

  for (var code = 0; code < unicodeLimit; code++) {
    if (code >= 0xd800 && code <= 0xdfff) continue;

    if (isPadding(code)) accepted.add(code);
  }

  print('whitespace\t${accepted.join(',')}');
}

String result(Object? value) {
  try {
    return parse(value).toString();
  } catch (error) {
    return 'error';
  }
}

bool isPadding(int code) {
  final padding = String.fromCharCode(code);

  return result('$padding$sample$padding') == '$sample';
}
