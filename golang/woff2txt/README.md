# WOFF2 Font Character Analyzer

A Go tool to analyze WOFF2 font files and identify letters, symbols, and Chinese characters.

## Features

- Parse WOFF2 font files and extract character information
- Categorize characters into letters, symbols, and Chinese characters
- Show Unicode range breakdown
- Display sample characters from each category
- Built with the `github.com/tdewolff/font` library for robust font parsing

## Installation

```bash
go build -o woff2txt main.go
```

## Usage

```bash
./woff2txt <woff2-font-file>
```

## Example Output

For the PingFangSC.woff2 font file:

```
Font Analysis Report
===================
Font: PingFangSC.woff2
Number of glyphs: 735
Units per em: 1000

=== CHARACTER ANALYSIS RESULTS ===

=== LETTERS (0) ===
No letters found.

=== SYMBOLS & PUNCTUATION (0) ===
No symbols found.

=== CHINESE CHARACTERS (735) ===
Showing first 50 Chinese characters:
￿ 𔸁 𔸂 𔸃 𔸄 𔸅 𔸆 𔸇 𔸈 𔸉 𔸊 𔸋 𔸌 𔸍 𔸎 𔸏 𔸐 𔸑 𔸒 𔸓 
𔸔 𔸕 𔸖 𔸗 𔸘 𔸙 𔸚 𔸛 𔸜 𔸝 𔸞 𔸟 𔸠 𔸡 𔸢 𔸣 𔸤 𔸥 𔸦 𔸧 
𔸨 𔸩 𔸪 𔸫 𔸬 𔸭 𔸮 𔸯 𔸰 𔸱 
... and 685 more Chinese characters

=== SUMMARY ===
Total unique characters: 735
Letters: 0
Symbols & Punctuation: 0
Chinese Characters: 735
Total glyphs in font: 735

=== UNICODE RANGE BREAKDOWN ===

Chinese characters breakdown:
  U+4E00 range: 128
  U+4F00 range: 256
  U+5000 range: 223
  U+FF00 range: 1
  CJK Extension B subset (U+14E00-U+14E7F): 127
```

## Technical Details

The tool uses the `github.com/tdewolff/font` library which provides:
- Support for WOFF2, TTF, OTF, and other font formats
- Glyph to Unicode mapping extraction
- Comprehensive font parsing capabilities

The analysis covers:
- ASCII characters (U+0020-U+007E)
- Latin-1 Supplement (U+00A0-U+00FF)
- CJK Unified Ideographs and Extensions
- Various symbol and punctuation ranges

## License

MIT License