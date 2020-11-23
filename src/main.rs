mod error; 
mod encrypt;

use colored::*;
use clap::{Arg, App, SubCommand}; 


const LOGO: &str =  r#"
            _                  
 _ __   ___| | _______      __  ／l、        
| '_ \ / _ \ |/ / _ \ \ /\ / /ﾞ（ﾟ､ ｡ ７ 
| | | |  __/   <  __/\ V  V /　 l、ﾞ ~ヽ  
|_| |_|\___|_|\_\___| \_/\_/　  じしf_, )ノ  
"#;



fn main() {
    let logotxt: ColoredString = String::from(LOGO).magenta();
    let nekew= App::new(LOGO)
        .version("0.1.0")
        .author("Xavier F. <xavier.fernandez92@myhunter.cuny.edu>")
        .about("a feline themed file encryption app ₍⸍⸌̣ʷ̣̫⸍̣⸌₎")
        .arg(Arg::with_name("encrypt")
            .short("en")
            .long("encrypt")
            .help("takes in a file for encryption")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("decrypt")
            .short("de")
            .long("decrypt")
            .help("takes in a file for decryption")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .short("pass")
            .long("password")
            .help("password for encryption/decryption")
            .value_name("PASS")
            .takes_value(true))
        .arg(Arg::with_name("scratch")
            .short("sc")
            .long("destroy")
            .help("optional flag to destroy the original file after encryption")
            .takes_value(false)
            .required(false)
            )
        .get_matches();
    
}
