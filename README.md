# dotdash-rs

The Rust version of my Morse Translation CLI. 

## Installation
>[!NOTE]
> If you wish to compile from source, you should have cargo and Rust installed on your system

### Linux/macOS

#### Compilation from source
Just paste the following code into your terminal :)
```bash
git clone https://github.com/epiGnosko/dotdash.git
cd dotdash # Navigating into the cloned repo
chmod +x source_install.sh
./source_install.sh
```

#### Grabbing the binary
Download the binary from the releases section on GitHub and place it in ```/usr/bin```

### Windows(WIP)

## Usage

### Encoding to Morse code
 - write the text to the stdin
    ```
    $ ./dotdash 
    hello world
    .... . .-.. .-.. --- / .-- --- .-. .-.. -..
    isn't this beautiful
    .. ... -. .----. - / - .... .. ... / -... . .- ..- - .. ..-. ..- .-..
    ```
 - Pass the text to the input stream
   ```
   $ echo "hello world" | ./dotdash 
   .... . .-.. .-.. --- / .-- --- .-. .-.. -..
   ```
 - redirect the input stream from a file
   ```
   $ echo "hello wold" > temp.txt
   $ ./dotdash < temp.txt 
   .... . .-.. .-.. --- / .-- --- .-.. -..
   ```
   
### Decoding from morse code
 - write the code to the stdin
    ```
    $ ./dotdash -d
    .... . .-.. .-.. --- / .-- --- .-. .-.. -..
    HELLO WORLD
    ```
 - Pass the code to the input stream
   ```
   $ echo ".... . .-.. .-.. --- / .-- --- .-. .-.. -.." | ./dotdash -d
   HELLO WORLD
   ```
 - redirect the input stream from a file
   ```
   $ echo ".... . .-.. .-.. --- / .-- --- .-.. -.." > temp.txt
   $ ./dotdash -d < temp.txt 
   HELLO WORLD
   ```

## Uninstallation

- Simply remove the binary from ```/usr/bin```
  ```bash
  sudo rm /usr/bin/dotdash
  ```
