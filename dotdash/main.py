#!/usr/bin/env python3

# Finalized Python version, no more bugs. Planned refactor to Rust.

import morse
import sys
import argparse

def main():
    parser = argparse.ArgumentParser(description="A CLI Utility to translate from Morse code to Text.")
    parser.add_argument("-r", action="store_true", help = "reverse the translation, convert text to Morse code instead")
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

                print(result, end="")
        except KeyboardInterrupt: # Handle Ctrl+C as well
            pass

    else:
        if args.r:
            result = morse.TexttoMorse(args.input)
        else:
            result = morse.MorsetoText(args.input)

        print(result, end="")

if __name__ == "__main__":
    main()
