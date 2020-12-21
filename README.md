![nekew-logo](nekew_png.png) 

## nekew 
Nekew is a feline themed cli file encryption tool which uses libsodium's [crypto secretstream encryption](https://libsodium.gitbook.io/doc/secret-key_cryptography/secretstream) 
 
## how to build

### prerequisutes: 
- A working [rust installation](https://www.rust-lang.org/tools/install)
- LINUX/OSX: A working installation of cc/clang for compiling [libsodium](https://libsodium.gitbook.io/doc/) and linking it to [sodiumoxide](https://crates.io/crates/sodiumoxide) 
- WINDOWS: you should have the [MSVC compiler](https://visualstudio.microsoft.com/vs/features/cplusplus/) installed for compiling libsodium-sys and linking it to sodiumoxide



> its been tested on windows/linux/osx now ฅ(＾・ω・＾ฅ) 
> slight caveat tho for windows, some of the cute cat kaomoji don't render properly in terminal unless you're using something like alacritty with a full monospaced font instead ♡(ﾐ ᵕ̣̣̣̣̣̣ ﻌ ᵕ̣̣̣̣̣̣ ﾐ)ﾉ
### compiling 
```shell 
 cargo build --bin nekew --color always --release
```
after this move the binary "nekew" from ``` /target/release ``` to ``` /usr/bin ``` on linux or ``` /usr/local/bin ``` on mac 

i'm not really sure where to add it on windows .....

- if compilation fails first check that you actually have a working c compiler and rust installed
- if it still fails after this delete the target directory and recompile

as a little test of the program you can run this after compilation is done: 

```shell 
./target/release/nekew --input bingus.gif.nekew --kill false --mode decrypt  --out ./  --sensitive 
``` 
and enter the password ```1234567890``` , use something better for your own files please

## usage 
```shell 

            _                  
 _ __   ___| | _______      __  ／l、        
| '_ \ / _ \ |/ / _ \ \ /\ / /ﾞ（ﾟ､ ｡ ７ 
| | | |  __/   <  __/\ V  V /　 l、ﾞ ~ヽ  
|_| |_|\___|_|\_\___| \_/\_/　  じしf_, )ノ  
0.1.0
Xavier F. <https://github.com/Xavulu/nekew>
a feline themed file encryption app (=Φܫ Φ=)∫

USAGE:
    nekew [FLAGS] --input <FILE> --kill <TRUE/FALSE> --mode <ENCRYPT/DECRYPT> --out <DIR>

FLAGS:
    -h, --help         Prints help information
    -s, --sensitive    Use a better keygen for more sensitive data, memory intense
    -V, --version      Prints version information

OPTIONS:
    -i, --input <FILE>              takes in a file for encryption/decryption
    -k, --kill <TRUE/FALSE>         destroy the original file after encryption
    -m, --mode <ENCRYPT/DECRYPT>    choose to encrypt or decrypt a file
    -o, --out <DIR>                 output directory for file


``` 
[![asciicast](https://asciinema.org/a/377913.svg)](https://asciinema.org/a/377913) 

### special commands 
- "--kill": takes a true false value, when true the file is [shredded](https://en.wikipedia.org/wiki/Shred_(Unix)) and deleted from the file system post encryption 
- "--sensitive": when this flag is present a more secure albeit more memory intensive key-derivation algorithm is used in encryption 

### encryption command example
```shell 
nekew --input "path to file" --kill TRUE --mode encrypt --out "output directory" --sensitive 
``` 
### decryption command example
```shell 
nekew --input "path to file" --kill TRUE --mode decrypt --out "output directory" --sensitive 
``` 

### WARNINGS
- PLEASE remember your password for encryption otherwise you're basically just using ransomware on your files  
- If you use the sensitive flag during file encryption you must also use it during file decryption. The opposite is also true i.e if the flag isn't present during encryption it shouldnt be there for decryption. If you mess up with this it will throw an error and create an empty file  

## TODO 
- Implement compression for encryption cause the files can end up MASSSIVE otherwise 
- Make the kill argument a flag instead 
- Test on windows and add binary releases for windows/mac/linux 
- Make errors look better for the end user

₍˄·͈༝·͈˄*₎◞ ̑̑  (=Φܫ Φ=)∫  ฅ(＾・ω・＾ฅ)