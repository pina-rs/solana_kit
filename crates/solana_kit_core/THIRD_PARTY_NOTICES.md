# Third-party notices

The bounded radix/text codec behavior and retained test fixtures are derived
from the Dart Solana Kit numeric/string packages at commit
`0138bb956d2d81ccc0c56edc602401c0df2f2b46`. The original license follows.
The durable nonce preparation and recognition contracts use the same pinned
repository's transaction message sources. Their actual Dart oracle and synthetic
literal fixtures retain this notice in `verification/durable-nonce/DART-LICENSE`.
The selected union, contextual transform, dynamic tuple and record, UTF-16 radix,
integer, address collation, lookup metadata and constant contracts use the same
pinned repository's corresponding core, data-structure, string, number, address
and instruction packages. The Keccak test literals come from its core hash tests.
These adaptations retain the MIT notice below. Cryptographic execution uses the
separately licensed RustCrypto `sha3` dependency rather than copied hash code.

MIT License

Copyright (c) 2026 Ifiok Jr.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

The instruction planning contracts and retained fixtures are based on the same
pinned Dart repository's complete `solana_kit_instruction_plans` package. Its MIT
notice is also retained in `verification/instruction-plans/DART-LICENSE`.

The shared error numeric registry, domain rules, message templates and oracle
fixtures are derived from `packages/solana_kit_errors` at the same pinned Dart
commit. They retain the MIT license above. Generated definitions belong to
`src/errors/catalog`; bounded Rust context and cause ownership are Kit's adaptation.
