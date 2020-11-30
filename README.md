![nekew-logo](nekew_png.png) 

## nekew 
yet another wip from your boy xavier f. 
see the description, this doesn't really work yet but .....


## how to build

### prerequisutes: 
You should have rust installed on your computer as well as cc/clang as this project uses 
sodiumoxide which requires them for linking to the c++ library libsodium 
this should work on all unix like os, I dont use windows so idk about that.....

### debug
```
cargo build --color always && ./target/debug/nekew 
``` 

### release 
``` 
 cargo build --bin nekew --color always --release
```

### usage 
``` 
            _                  
 _ __   ___| | _______      __  ／l、        
| '_ \ / _ \ |/ / _ \ \ /\ / /ﾞ（ﾟ､ ｡ ７ 
| | | |  __/   <  __/\ V  V /　 l、ﾞ ~ヽ  
|_| |_|\___|_|\_\___| \_/\_/　  じしf_, )ノ  
0.1.0
Xavier F. <https://github.com/Xavulu/nekew>
a feline themed file encryption app (=Φܫ Φ=)∫

USAGE:
    nekew --input <FILE> --kill <TRUE/FALSE> --mode <encrypt/decrypt> --out <DIR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>              takes in a file for encryption/decryption
    -k, --kill <TRUE/FALSE>         destroy the original file after encryption
    -m, --mode <encrypt/decrypt>    choose to encrypt or decrypt a file
    -o, --out <DIR>                 output directory for file
```