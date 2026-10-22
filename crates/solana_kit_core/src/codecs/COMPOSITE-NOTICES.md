# Composite codec provenance

The framing, offset, fixed-size, reversal, structured bit-array, value mapping,
padding, sentinel, literal and dependent
record behavior was adapted from Solana Kit Dart at commit
0138bb956d2d81ccc0c56edc602401c0df2f2b46. Original packages are
solana_kit_codecs_core and solana_kit_codecs_data_structures. The Rust API uses
concrete types and shared bounded cursors. The scoped receipt records deliberate
contract differences. Fixtures are produced by the original pinned code.

The upstream MIT notice follows. Project-owned implementation remains Unlicense.
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
