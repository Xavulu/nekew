


mod encrypt;
use encrypt::{encrypt, decrypt, verify};

use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::io::{Read, Write, Seek, SeekFrom};
use std::time::Instant;

use colored::*;
use clap::{Arg, App, SubCommand, AppSettings}; 
use color_eyre::eyre::{Report, Result, WrapErr};
use tracing::{info, instrument}; 
use rpassword;
use file_shred::shred_file;
use paris::{LogIcon, Logger};

const EXTENSION: &str = ".nekew";
const EXISTS: &str ="enc_";

const LOGO: &str =  r#"
            _                  
 _ __   ___| | _______      __  ／l、        
| '_ \ / _ \ |/ / _ \ \ /\ / /ﾞ（ﾟ､ ｡ ７ 
| | | |  __/   <  __/\ V  V /　 l、ﾞ ~ヽ  
|_| |_|\___|_|\_\___| \_/\_/　  じしf_, )ノ 
"#;


#[instrument]
fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let mut log = Logger::new();
    let nekew= App::new(LOGO)
        .setting(AppSettings::ColorAlways)
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("\n0.1.0")
        .author("Xavier F. <https://github.com/Xavulu/nekew>")
        .about(r#"a feline themed file encryption app (=Φܫ Φ=)∫"#)
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .help("choose to encrypt or decrypt a file")
            .value_name("ENCRYPT/DECRYPT")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("takes in a file for encryption/decryption")
            .value_name("FILE")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("output directory for file")
            .value_name("DIR")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("kill")
            .short("k")
            .long("kill")
            .help("destroy the original file after encryption")
            .value_name("TRUE/FALSE")
            .takes_value(true)
            .required(true)
            )
        .arg(Arg::with_name("sensitive")
            .short("s")
            .long("sensitive")
            .help("Use a better keygen for more sensitive data, memory intense")    
            )
        .get_matches();
    //path and file validation
    let input = nekew.value_of("input").unwrap();
    let output =  nekew.value_of("out").unwrap();
    let infile = Path::new(input); 
    let outpath = Path::new(output);
    let sensitive = nekew.is_present("sensitive");

    if infile.is_dir() || !infile.is_file() || !infile.exists(){ 
        eprintln!("{}", "Invalid input file, please specify path to a file (=xܫ x=)∫".red());
        exit(1);
    }
    if !outpath.is_dir() || outpath.is_file() || !outpath.exists(){
        eprintln!("{}", "Invalid output directory, please specify output path (=xܫ x=)∫".red());
        exit(1);
    } 
    let _infile = infile;
    //password retrieval and validation
    let password = rpassword::prompt_password_stdout("Please enter a password with 10 or more characters: ")
        .expect("couldn't get a password"); 
    if password.len() < 10 { 
        eprintln!("{}", "Your password should be more than 10 chracters (=xܫ x=)∫".red());
        exit(10);
    }  
    let passcheck = rpassword::prompt_password_stdout("Confirm your password: ")
    .expect("couldn't get a password"); 
    if passcheck.len() != password.len() || passcheck != password { 
        eprintln!("{}", "The passwords don't match (=xܫ x=)∫".red());
        exit(1);
    }

    //encryption and associated file creation/checking stuff 
    let mode = nekew.value_of("mode").unwrap(); 
    let mut cryptin = match File::open(&infile) { 
        Err(why) => panic!("{} {}: {} ", "couldn't open".red(), infile.display(), why), 
        Ok(file) => file,
    };
    
    let proctime = Instant::now();
    
    match mode { 
        "encrypt" | "ENCRYPT" | "Encrypt" => {  
            log.loading("<blue>beginning file encryption</>  <magenta>ฅ(＾・ω・＾ฅ)</>");
            let origin = Path::new(& _infile).file_name().unwrap().to_str().unwrap();
            let out = format!("{}{}{}", outpath.display(), origin ,String::from(EXTENSION));
            let outexist = format!("{}{}{}{}", outpath.display() ,String::from(EXISTS) , origin, String::from(EXTENSION));
            let enc = Path::new(&out); //holy shit is string formatting bizzare here
            let enc_exists = Path::new(&outexist);
            //let mut filetest = File::create(enc_exists).unwrap(); 

            match verify(&mut cryptin){ 
                true => {
                    eprintln!("{}", "\nthis file is already encrypted you know".red());
                    exit(1);
                }, 
                false => { 
                    if enc.exists() { 
                        let mut f_exists = match File::create(&enc_exists) {
                            Err(why) => panic!("{} {}: {} ", "couldn't create output ".red(), enc_exists.display(), why), 
                            Ok(file) => file,
                        };
                        let exists_result = encrypt(&mut cryptin, &mut f_exists, password.as_str(), sensitive);
                        match exists_result { 
                            Ok(()) => println!("{}", "\nencryption succesful ₍˄·͈༝·͈˄*₎◞ ".blue()), 
                            Err(failed) => panic!("Encryption error: {:?}", failed),
                        }
                    } 
                    else { 
                        let mut f_new = match File::create(&enc){
                            Err(why) => panic!("{} {}: {} ", "couldn't create output ".red(), enc.display(), why), 
                            Ok(file) => file,
                        };  
                        let enc_result = encrypt(&mut cryptin, &mut f_new, password.as_str(), sensitive);
                        match enc_result { 
                            Ok(()) => println!("{}", "\nencryption succesful ₍˄·͈༝·͈˄*₎◞ ".blue()), 
                            Err(failed) => panic!("Encryption error: {:?}", failed),
                        }
                    }
                },
            }
            
        }, 
        "decrypt" | "DECRYPT" | "Decrypt" => { 
            log.loading("<blue>beginning file decryption</>  <magenta>ฅ(＾・ω・＾ฅ)</>");
            match verify(&mut cryptin){ 
                true => { 
                    let strip = infile.to_str().unwrap().to_string(); 
                    let prepend = format!("{}dec_{}",String::from(output) , strip[..strip.len() - EXTENSION.len()].to_string());
                    let dec_file = Path::new(&prepend); 
                    let mut dec = match File::create(&dec_file){ 
                        Err(why) => panic!("{} {}: {} ", "couldn't create output ".red(), dec_file.display(), why), 
                        Ok(file) => file,
                    };
    
                    let dec_result = decrypt(&mut cryptin, &mut dec, password.as_str(), sensitive);
                    match dec_result { 
                        Ok(()) => println!("{}", "\ndecryption succesful ₍˄·͈༝·͈˄*₎◞ ".blue()), 
                        Err(failed) => panic!("Encryption error: {:?}", failed),
                    }
                }, 
                false => { 
                    eprintln!("{}", "this file is either not encrypted or encrypted by another program (=xܫ x=)∫".red()); 
                    exit(1);
                },
            }
        }, 
        _ => { 
            eprintln!("{} {}", mode , "is not a valid mode sorry (=xܫ x=)∫".red()); 
            exit(1);
        },
    }

    let finished = proctime.elapsed();
    println!("\n{} {:#?}","completed in:".magenta() , finished);

    //confirmation for file shredding and destruction  
    let shred = nekew.value_of("kill").unwrap(); 
    match shred { 
        "true" | "TRUE" | "True" => {
            log.warn("<yellow>The original file will be</> <red>PERMANENTLY</> <yellow>deleted/shredded</> (^._.^)");
            let shredded = file_shred::shred_file(infile);
            match shredded { 
                Ok(()) => {
                    println!("{} was succesfully shredded (=Φܫ Φ=)∫", infile.display()); 
                }, 
                Err(fail) => {
                    eprintln!("{}",fail.red()); 
                    exit(1);
                } ,
            }
        },
        _ => println!("\n{} ", "no file shredding".green()),
    }

    println!("goodbye {} {}", "<3".red().bold(), "₍˄·͈༝·͈˄*₎◞ ̑̑  (=Φܫ Φ=)∫  ฅ(＾・ω・＾ฅ)".magenta());

    Ok(())
}


