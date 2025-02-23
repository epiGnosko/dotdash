# Morse Translator

A program which can translate to and from morse code. accepts command line arguments as morse and returns text output and vice versa.

## Installation

Paste the following commands in a linux terminal

```bash
git clone https://github.com/Gurmukh-Singh-4253/dotdash.git
cd dotdash
sudo ./install.sh
```

> [!NOTE]
> You should have a working python3 installation to run the script
> This script only works on linux systems for now.

## Usage

```
usage: dotdash [-h] [-r] [input]

A CLI Utility to translate from Morse code to Text.

positional arguments:
  input       The text or morse code to translate. If not provided, read from
              the stdin

options:
  -h, --help  show this help message and exit
  -r          reverse the translation, convert text to Morse code instead
```

You can give input in any format:
- Command Line arguments
    ```bash
    $ dotdash -r "Hello world"
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    ```
- Read from stdin
    ```bash
    $ dotdash -r
    Hello world
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    ```
- pipe text to stdin
    ```bash
    $ echo "Hello world" | dotdash -r
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    ```
- read from file 
    ```bash
    $ echo "Hello world" > hello.txt
    $ dotdash -r < hello.txt 
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    ```
