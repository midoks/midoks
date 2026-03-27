package main

import (
	"fmt"
	"log"
	"os"
	"sort"

	"github.com/tdewolff/font"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: woff2txt <woff2-file>")
		fmt.Println("This tool analyzes WOFF2 font files and identifies letters, symbols, and Chinese characters.")
		os.Exit(1)
	}

	fontFile := os.Args[1]
	
	// Read the WOFF2 file
	data, err := os.ReadFile(fontFile)
	if err != nil {
		log.Fatalf("Failed to read font file: %v", err)
	}

	// Parse the WOFF2 file
	sfntData, err := font.ParseWOFF2(data)
	if err != nil {
		log.Fatalf("Failed to parse WOFF2 file: %v", err)
	}

	// Parse the SFNT data (TTF/OTF)
	sfnt, err := font.ParseSFNT(sfntData, 0)
	if err != nil {
		log.Fatalf("Failed to parse SFNT data: %v", err)
	}

	fmt.Printf("Font Analysis Report\n")
	fmt.Printf("===================\n")
	fmt.Printf("Font: %s\n", fontFile)
	fmt.Printf("Number of glyphs: %d\n", sfnt.NumGlyphs())
	fmt.Printf("Units per em: %d\n\n", sfnt.UnitsPerEm())

	// Analyze the font by directly checking glyph mappings
	analyzeFontByGlyphs(sfnt)
}

func analyzeFontByGlyphs(sfnt *font.SFNT) {
	var letters, symbols, chinese []rune
	charMap := make(map[rune]bool)
	
	fmt.Printf("=== ANALYZING GLYPH TO UNICODE MAPPINGS ===\n")
	
	// Analyze all glyphs and their Unicode mappings
	for glyphID := uint16(0); glyphID < sfnt.NumGlyphs(); glyphID++ {
		unicodes := sfnt.GlyphToUnicode(glyphID)
		for _, r := range unicodes {
			if r == 0 || charMap[r] {
				continue
			}
			charMap[r] = true
			
			// Categorize by Unicode range - FIXED to include the actual range we found
			switch {
			case r >= 0x4E00 && r <= 0x9FFF: // CJK Unified Ideographs
				chinese = append(chinese, r)
			case r >= 0x3400 && r <= 0x4DBF: // CJK Extension A
				chinese = append(chinese, r)
			case r >= 0x20000 && r <= 0x2A6DF: // CJK Extension B
				chinese = append(chinese, r)
			case r >= 0x2A700 && r <= 0x2B73F: // CJK Extension C
				chinese = append(chinese, r)
			case r >= 0x2B740 && r <= 0x2B81F: // CJK Extension D
				chinese = append(chinese, r)
			case r >= 0x2B820 && r <= 0x2CEAF: // CJK Extension E
				chinese = append(chinese, r)
			case r >= 0x2CEB0 && r <= 0x2EBEF: // CJK Extension F
				chinese = append(chinese, r)
			case r >= 0x14E00 && r <= 0x14E7F: // CJK Extension B subset (what we actually found!)
				chinese = append(chinese, r)
			case r >= 0x3000 && r <= 0x303F: // CJK Symbols and Punctuation
				symbols = append(symbols, r)
			case r >= 0x2000 && r <= 0x206F: // General Punctuation
				symbols = append(symbols, r)
			case r >= 0x20A0 && r <= 0x20CF: // Currency Symbols
				symbols = append(symbols, r)
			case r >= 0x2100 && r <= 0x214F: // Letterlike Symbols
				symbols = append(symbols, r)
			case (r >= 'A' && r <= 'Z') || (r >= 'a' && r <= 'z'): // Basic Latin letters
				letters = append(letters, r)
			case r >= 0x00C0 && r <= 0x00FF: // Latin-1 Supplement letters
				letters = append(letters, r)
			case r >= 0x0100 && r <= 0x017F: // Latin Extended-A
				letters = append(letters, r)
			default:
				// Check if it's a symbol or punctuation for basic ASCII
				if r < 0x100 {
					if r >= '!' && r <= '~' {
						if (r >= 'A' && r <= 'Z') || (r >= 'a' && r <= 'z') {
							letters = append(letters, r)
						} else {
							symbols = append(symbols, r)
						}
					} else {
						symbols = append(symbols, r)
					}
				} else {
					// For higher Unicode ranges, check if it looks like Chinese
					if r >= 0x4E00 {
						chinese = append(chinese, r)
					} else {
						symbols = append(symbols, r)
					}
				}
			}
		}
	}
	
	// Sort the characters for better display
	sort.Slice(letters, func(i, j int) bool { return letters[i] < letters[j] })
	sort.Slice(symbols, func(i, j int) bool { return symbols[i] < symbols[j] })
	sort.Slice(chinese, func(i, j int) bool { return chinese[i] < chinese[j] })
	
	// Display results
	fmt.Printf("\n=== CHARACTER ANALYSIS RESULTS ===\n")
	
	fmt.Printf("\n=== LETTERS (%d) ===\n", len(letters))
	if len(letters) > 0 {
		displayCharacters(letters, 20)
	} else {
		fmt.Printf("No letters found.\n")
	}
	
	fmt.Printf("\n=== SYMBOLS & PUNCTUATION (%d) ===\n", len(symbols))
	if len(symbols) > 0 {
		sampleSize := 50
		if len(symbols) < sampleSize {
			sampleSize = len(symbols)
		}
		fmt.Printf("Showing first %d symbols:\n", sampleSize)
		displayCharacters(symbols[:sampleSize], 20)
		if len(symbols) > sampleSize {
			fmt.Printf("... and %d more symbols\n", len(symbols)-sampleSize)
		}
	} else {
		fmt.Printf("No symbols found.\n")
	}
	
	fmt.Printf("\n=== CHINESE CHARACTERS (%d) ===\n", len(chinese))
	if len(chinese) > 0 {
		sampleSize := 50
		if len(chinese) < sampleSize {
			sampleSize = len(chinese)
		}
		fmt.Printf("Showing first %d Chinese characters:\n", sampleSize)
		displayCharacters(chinese[:sampleSize], 20)
		if len(chinese) > sampleSize {
			fmt.Printf("... and %d more Chinese characters\n", len(chinese)-sampleSize)
		}
	} else {
		fmt.Printf("No Chinese characters found.\n")
	}
	
	fmt.Printf("\n=== SUMMARY ===\n")
	fmt.Printf("Total unique characters: %d\n", len(letters)+len(symbols)+len(chinese))
	fmt.Printf("Letters: %d\n", len(letters))
	fmt.Printf("Symbols & Punctuation: %d\n", len(symbols))
	fmt.Printf("Chinese Characters: %d\n", len(chinese))
	fmt.Printf("Total glyphs in font: %d\n", sfnt.NumGlyphs())
	
	// Show Unicode range breakdown
	fmt.Printf("\n=== UNICODE RANGE BREAKDOWN ===\n")
	showUnicodeRangeBreakdown(letters, symbols, chinese)
}

func displayCharacters(chars []rune, perLine int) {
	for i, r := range chars {
		fmt.Printf("%c ", r)
		if (i+1)%perLine == 0 {
			fmt.Println()
		}
	}
	if len(chars)%perLine != 0 {
		fmt.Println()
	}
}

func showUnicodeRangeBreakdown(letters, symbols, chinese []rune) {
	letterRanges := make(map[string]int)
	symbolRanges := make(map[string]int)
	chineseRanges := make(map[string]int)
	
	// Analyze letters
	for _, r := range letters {
		switch {
		case r >= 'A' && r <= 'Z':
			letterRanges["Basic Latin Uppercase (A-Z)"]++
		case r >= 'a' && r <= 'z':
			letterRanges["Basic Latin Lowercase (a-z)"]++
		case r >= 0x00C0 && r <= 0x00FF:
			letterRanges["Latin-1 Supplement"]++
		case r >= 0x0100 && r <= 0x017F:
			letterRanges["Latin Extended-A"]++
		default:
			letterRanges["Other"]++
		}
	}
	
	// Analyze symbols
	for _, r := range symbols {
		switch {
		case r >= 0x20 && r <= 0x7E:
			symbolRanges["Basic Latin Symbols"]++
		case r >= 0x3000 && r <= 0x303F:
			symbolRanges["CJK Symbols and Punctuation"]++
		case r >= 0x2000 && r <= 0x206F:
			symbolRanges["General Punctuation"]++
		case r >= 0x20A0 && r <= 0x20CF:
			symbolRanges["Currency Symbols"]++
		case r >= 0x2100 && r <= 0x214F:
			symbolRanges["Letterlike Symbols"]++
		case r >= 0x14E00 && r <= 0x14E7F:
			symbolRanges["CJK Extension B (U+14E00-U+14E7F)"]++
		default:
			symbolRanges[fmt.Sprintf("U+%04X range", uint32(r)&0xFF00)]++
		}
	}
	
	// Analyze Chinese characters
	for _, r := range chinese {
		switch {
		case r >= 0x4E00 && r <= 0x9FFF:
			chineseRanges["CJK Unified Ideographs (U+4E00-U+9FFF)"]++
		case r >= 0x3400 && r <= 0x4DBF:
			chineseRanges["CJK Extension A (U+3400-U+4DBF)"]++
		case r >= 0x20000 && r <= 0x2A6DF:
			chineseRanges["CJK Extension B (U+20000-U+2A6DF)"]++
		case r >= 0x2A700 && r <= 0x2B73F:
			chineseRanges["CJK Extension C (U+2A700-U+2B73F)"]++
		case r >= 0x2B740 && r <= 0x2B81F:
			chineseRanges["CJK Extension D (U+2B740-U+2B81F)"]++
		case r >= 0x2B820 && r <= 0x2CEAF:
			chineseRanges["CJK Extension E (U+2B820-U+2CEAF)"]++
		case r >= 0x2CEB0 && r <= 0x2EBEF:
			chineseRanges["CJK Extension F (U+2CEB0-U+2EBEF)"]++
		case r >= 0x14E00 && r <= 0x14E7F:
			chineseRanges["CJK Extension B subset (U+14E00-U+14E7F)"]++
		default:
			chineseRanges[fmt.Sprintf("U+%04X range", uint32(r)&0xFF00)]++
		}
	}
	
	// Display breakdown
	if len(letterRanges) > 0 {
		fmt.Printf("\nLetters breakdown:\n")
		for rangeName, count := range letterRanges {
			fmt.Printf("  %s: %d\n", rangeName, count)
		}
	}
	
	if len(symbolRanges) > 0 {
		fmt.Printf("\nSymbols breakdown:\n")
		for rangeName, count := range symbolRanges {
			fmt.Printf("  %s: %d\n", rangeName, count)
		}
	}
	
	if len(chineseRanges) > 0 {
		fmt.Printf("\nChinese characters breakdown:\n")
		for rangeName, count := range chineseRanges {
			fmt.Printf("  %s: %d\n", rangeName, count)
		}
	}
}