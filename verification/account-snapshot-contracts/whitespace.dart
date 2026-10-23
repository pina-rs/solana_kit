import 'package:solana_kit_accounts/solana_kit_accounts.dart';

void main() {
  for (final code in [
    0,
    9,
    10,
    11,
    12,
    13,
    32,
    133,
    160,
    5760,
    8192,
    8199,
    8232,
    8233,
    8239,
    8287,
    12288,
    65279,
  ]) {
    final padding = String.fromCharCode(code);
    try {
      final header = parseBaseAccount({
        'executable': false,
        'lamports': '${padding}1$padding',
        'space': 1,
        'owner': '11111111111111111111111111111111',
      });
      print('$code|${header.lamports.value}');
    } catch (error) {
      print('$code|error');
    }
  }
}
