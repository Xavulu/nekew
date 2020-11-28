#![allow(dead_code, unused_imports)]


mod encrypt;

use colored::*;
use clap::{Arg, App, SubCommand, AppSettings}; 

const EXTENSION: &str = ".nekew";

const LOGO: &str =  r#"
            _                  
 _ __   ___| | _______      __  ／l、        
| '_ \ / _ \ |/ / _ \ \ /\ / /ﾞ（ﾟ､ ｡ ７ 
| | | |  __/   <  __/\ V  V /　 l、ﾞ ~ヽ  
|_| |_|\___|_|\_\___| \_/\_/　  じしf_, )ノ 
"#;



fn main() {
    /*let logotxt: ColoredString = String::from(LOGO).magenta();*/
    let nekew= App::new(LOGO)
        .setting(AppSettings::ColorAlways)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("\n0.1.0")
        .author("Xavier F. <https://github.com/Xavulu/nekew>")
        .about(r#"a feline themed file encryption app 🐈"#)
        .arg(Arg::with_name("encrypt")
            .short("e")
            .long("encrypt")
            .help("takes in a file for encryption")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("decrypt")
            .short("d")
            .long("decrypt")
            .help("takes in a file for decryption")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("p")
            .long("password")
            .help("password for encryption/decryption")
            .value_name("PASS")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("mangle")
            .short("m")
            .long("mangle")
            .help("destroy the original file after encryption")
            .value_name("KILL")
            .takes_value(true)
            .required(true)
            )
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("output directory for encrypted/decrypted file")
            .value_name("DIR")
            .takes_value(true)
            .required(true))
        .get_matches();
    
}
