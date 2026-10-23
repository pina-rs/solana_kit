import 'package:solana_kit_accounts/solana_kit_accounts.dart';

void main() {
  for (final value in [
    '1',
    '+1',
    '01',
    ' 1',
    '1 ',
    '\t1\n',
    '0x10',
    '0Xff',
    '+0x10',
    '-0',
    '-1',
    '1.0',
    '1e2',
    '',
    '0x',
    '1_0',
    '18446744073709551615',
    '18446744073709551616',
    true,
    null,
  ]) {
    try {
      final parsed = parseBaseAccount({
        'executable': false,
        'lamports': value,
        'owner': '11111111111111111111111111111111',
        'space': value,
      });
      print(
        '${value.runtimeType}|${value.toString().replaceAll('\n', '\\n').replaceAll('\t', '\\t')}|${parsed.lamports.value}|${parsed.space}',
      );
    } catch (error) {
      print(
        '${value.runtimeType}|${value.toString().replaceAll('\n', '\\n').replaceAll('\t', '\\t')}|error',
      );
    }
  }
}
