#!/usr/bin/env python3

import morse
import sys
import argparse

parser = argparse.ArgumentParser(description="Translate between Morse code and text")
parser.add_argument("-r", action="store_true", help = "convert text to Morse code instead")
parser.add_argument("input", nargs="?", type=str, default=None, help = "The text or morse code to translate. If not provided, read from the stdin")
args = parser.parse_args()

if args.input is None:
    lines = []

    try:
        for line in sys.stdin:
            if args.r:
                result = morse.TexttoMorse(line)
            else:
                result = morse.MorsetoText(line)

            print(result)
    except KeyboardInterrupt: # Handle Ctrl+C as well
        pass

else:
    if args.r:
        result = morse.TexttoMorse(args.input)
    else:
        result = morse.MorsetoText(args.input)

    print(result)
