#!/usr/bin/env python3
"""
WOFF2 Font Text Extractor using OCR
Extracts text from WOFF2 font files by converting glyphs to images and using OCR
"""

import os
import sys
import json
from pathlib import Path
from typing import Dict, List, Optional
import logging

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

try:
    from fontTools.ttLib import TTFont
    from fontTools.unicode import Unicode
    from PIL import Image, ImageDraw, ImageFont
    from cnocr import CnOcr
except ImportError as e:
    logger.error(f"Missing required dependencies: {e}")
    logger.info("Please install: pip install fonttools pillow cnocr")
    sys.exit(1)


class Woff2TextExtractor:
    """Extract text from WOFF2 font files using OCR"""
    
    def __init__(self, font_size: int = 64, image_size: tuple = (128, 128)):
        """
        Initialize the extractor
        
        Args:
            font_size: Size of characters to render
            image_size: Size of output images (width, height)
        """
        self.font_size = font_size
        self.image_size = image_size
        self.ocr = CnOcr()
        self.char_mapping = {}
        
    def load_font(self, font_path: str) -> TTFont:
        """
        Load WOFF2 font file
        
        Args:
            font_path: Path to WOFF2 file
            
        Returns:
            TTFont object
        """
        try:
            font = TTFont(font_path)
            logger.info(f"Successfully loaded font: {font_path}")
            return font
        except Exception as e:
            logger.error(f"Failed to load font {font_path}: {e}")
            raise
    
    def get_unicode_chars(self, font: TTFont) -> List[int]:
        """
        Extract all Unicode characters from font
        
        Args:
            font: TTFont object
            
        Returns:
            List of Unicode code points
        """
        unicode_chars = []
        
        # Get character mapping from the font
        for table in font['cmap'].tables:
            if table.isUnicode():
                for code_point in table.cmap.keys():
                    unicode_chars.append(code_point)
        
        # Remove duplicates and sort
        unicode_chars = sorted(list(set(unicode_chars)))
        logger.info(f"Found {len(unicode_chars)} unique Unicode characters")
        
        return unicode_chars
    
    def char_to_image(self, char_code: int, font_path: str) -> Optional[Image.Image]:
        """
        Convert a character to an image using the font
        
        Args:
            char_code: Unicode code point
            font_path: Path to font file
            
        Returns:
            PIL Image object or None if failed
        """
        try:
            # Create blank image with white background
            image = Image.new('L', self.image_size, 255)
            draw = ImageDraw.Draw(image)
            
            # Load font
            font = ImageFont.truetype(font_path, self.font_size)
            
            # Get character
            char = chr(char_code)
            
            # Calculate text position to center it
            bbox = draw.textbbox((0, 0), char, font=font)
            text_width = bbox[2] - bbox[0]
            text_height = bbox[3] - bbox[1]
            
            x = (self.image_size[0] - text_width) // 2
            y = (self.image_size[1] - text_height) // 2
            
            # Draw character
            draw.text((x, y), char, font=font, fill=0)
            
            return image
            
        except Exception as e:
            logger.warning(f"Failed to create image for character U+{char_code:04X}: {e}")
            return None
    
    def ocr_recognize(self, image: Image.Image) -> Optional[str]:
        """
        Recognize text in image using OCR
        
        Args:
            image: PIL Image object
            
        Returns:
            Recognized text or None if failed
        """
        try:
            # Convert PIL image to format expected by CnOCR
            result = self.ocr.ocr(image)
            
            if result and len(result) > 0:
                # Extract text from OCR result
                text = ''.join([item['text'] for item in result if 'text' in item])
                return text.strip()
            
            return None
            
        except Exception as e:
            logger.warning(f"OCR recognition failed: {e}")
            return None
    
    def extract_text_mapping(self, font_path: str, output_dir: str = "output") -> Dict[str, str]:
        """
        Extract text mapping from WOFF2 font file
        
        Args:
            font_path: Path to WOFF2 file
            output_dir: Directory to save intermediate images and results
            
        Returns:
            Dictionary mapping character codes to recognized text
        """
        logger.info(f"Starting text extraction from {font_path}")
        
        # Create output directory
        Path(output_dir).mkdir(exist_ok=True)
        images_dir = Path(output_dir) / "char_images"
        images_dir.mkdir(exist_ok=True)
        
        # Load font
        font = self.load_font(font_path)
        
        # Get all Unicode characters
        unicode_chars = self.get_unicode_chars(font)
        
        # Process each character
        mapping = {}
        successful_recognitions = 0
        
        for i, char_code in enumerate(unicode_chars):
            if i % 100 == 0:
                logger.info(f"Processing character {i}/{len(unicode_chars)}")
            
            # Convert character to image
            image = self.char_to_image(char_code, font_path)
            if image is None:
                continue
            
            # Save image for debugging
            image_path = images_dir / f"char_U+{char_code:04X}.png"
            image.save(image_path)
            
            # Recognize character using OCR
            recognized_text = self.ocr_recognize(image)
            
            if recognized_text:
                char = chr(char_code)
                mapping[char] = recognized_text
                successful_recognitions += 1
                logger.debug(f"U+{char_code:04X} ({char}) -> {recognized_text}")
            
        logger.info(f"Successfully recognized {successful_recognitions}/{len(unicode_chars)} characters")
        
        # Save mapping to JSON
        mapping_path = Path(output_dir) / "char_mapping.json"
        with open(mapping_path, 'w', encoding='utf-8') as f:
            json.dump(mapping, f, ensure_ascii=False, indent=2)
        
        self.char_mapping = mapping
        return mapping
    
    def get_extracted_text(self) -> str:
        """
        Get all extracted text as a single string
        
        Returns:
            Extracted text
        """
        if not self.char_mapping:
            return ""
        
        # Sort by character code for consistent output
        sorted_items = sorted(self.char_mapping.items(), key=lambda x: ord(x[0]))
        return ''.join([text for _, text in sorted_items])


def main():
    """Main function"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Extract text from WOFF2 font files using OCR")
    parser.add_argument("input_file", help="Path to WOFF2 font file")
    parser.add_argument("-o", "--output", default="output", help="Output directory (default: output)")
    parser.add_argument("-s", "--font-size", type=int, default=64, help="Font size for rendering (default: 64)")
    parser.add_argument("-i", "--image-size", type=int, nargs=2, default=[128, 128], 
                       help="Image size for character rendering (default: 128 128)")
    parser.add_argument("--save-text", action="store_true", help="Save extracted text to file")
    
    args = parser.parse_args()
    
    # Check if input file exists
    if not os.path.exists(args.input_file):
        logger.error(f"Input file not found: {args.input_file}")
        sys.exit(1)
    
    # Create extractor
    extractor = Woff2TextExtractor(
        font_size=args.font_size,
        image_size=tuple(args.image_size)
    )
    
    try:
        # Extract text mapping
        mapping = extractor.extract_text_mapping(args.input_file, args.output)
        
        # Get extracted text
        extracted_text = extractor.get_extracted_text()
        
        logger.info(f"Extracted {len(mapping)} character mappings")
        logger.info(f"Extracted text length: {len(extracted_text)}")
        
        # Save extracted text if requested
        if args.save_text:
            text_path = Path(args.output) / "extracted_text.txt"
            with open(text_path, 'w', encoding='utf-8') as f:
                f.write(extracted_text)
            logger.info(f"Extracted text saved to: {text_path}")
        
        # Print summary
        print(f"\n=== Extraction Summary ===")
        print(f"Font file: {args.input_file}")
        print(f"Characters processed: {len(mapping)}")
        print(f"Output directory: {args.output}")
        print(f"Extracted text preview: {extracted_text[:100]}...")
        
    except Exception as e:
        logger.error(f"Extraction failed: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()