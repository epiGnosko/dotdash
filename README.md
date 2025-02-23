# dotdash

A program which can translate to and from morse code. accepts command line arguments as morse and returns text output and vice versa.

## Installation

> [!NOTE]
> You should have a working python3 installation to run the script

### Linux/macOS

Paste the following commands in your terminal.

```bash
git clone https://github.com/Gurmukh-Singh-4253/dotdash.git
cd dotdash # Navigating into the cloned repo
```

now you can install either via pip or run the installation script provided

- Pip (recommended)
```bash
python3 -m venv venv 
source venv/bin/activate
pip install . # asking pip to install the command as a package in your system
dotdash --help # testing the installation
```

<p align='center'>
    OR
</p>

- Installation script
```bash
sudo ./install.sh # installs system-wide, will ask for your password
dotdash --help # testing the installation
```

### Windows (Experimental, I did not have a windows machine to test this on, raise an issue if it does not work)

Paste the underlying code into powershell

```powershell 
git clone https://github.com/Gurmukh-Singh-4253/dotdash.git
cd dotdash 
python -m venv venv
venv\Scripts\activate
pip install .
```

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
    $ dotdash
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    HELLO WORLD
    ```
- read from file 
    ```bash
    $ echo "Hello world" > hello.txt
    $ dotdash -r < hello.txt 
    .... . .-.. .-.. --- / .-- --- .-. .-.. -.. 
    ```
- pipe text to stdin
    ```bash
    $ echo ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." | dotdash
    HELLO WORLD
    ```

## Uninstall

To uninstall the utility

- If installed via installation script

```bash
sudo ./uninstall.sh # removes the utility from your system
```

- If installed via pip

```bash
pip uninstall dotdash
```
