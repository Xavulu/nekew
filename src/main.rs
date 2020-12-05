#![allow(dead_code, unused_imports)]


mod encrypt;
use encrypt::*;

use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;
use std::error::Error;
use std::io;
use std::io::{Read, Write, Seek, SeekFrom};
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
        .get_matches();
    //path and file validation
    let input = nekew.value_of("input").unwrap();
    let output =  nekew.value_of("out").unwrap();
    let infile = Path::new(input); 
    let outpath = Path::new(output);

    if infile.is_dir() || !infile.is_file() || !infile.exists(){ 
        eprintln!("{}", "Invalid input file, please specify path to a file (=xܫ x=)∫".red());
        exit(1);
    }
    if !outpath.is_dir() || outpath.is_file() || !outpath.exists(){
        eprintln!("{}", "Invalid output directory, please specify output path (=xܫ x=)∫".red());
        exit(1);
    } 

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
    match mode { 
        "encrypt" | "ENCRYPT" | "Encrypt" => {  }, 
        "decrypt" | "DECRYPT" | "Decrypt" => { }, 
        _ => { 
            eprintln!("{} {}", mode , "is not a valid mode sorry (=xܫ x=)∫".red()); 
            exit(1);
        },
    }

    //confirmation for file shredding and destruction  
    let shred = nekew.value_of("kill").unwrap(); 
    match shred { 
        "true" | "TRUE" | "True" => {
            log.warn("<yellow>The original file will be</> <red>PERMANENTLY</> <yellow>deleted/shredded</> (^._.^)");
            let shredded = file_shred::shred_file(infile);
            match shredded { 
                Ok(i) => {
                    println!("{} was succesfully shredded (=Φܫ Φ=)∫", infile.display()); 
                    i //not really needed :)
                }, 
                Err(fail) => {
                    eprintln!("{}",fail.red()); 
                    exit(1);
                } ,
            }
        },
        _ => println!("no file shredding "),
    }

    println!("goodbye {} {}", "<3".red().bold(), "₍˄·͈༝·͈˄*₎◞ ̑̑  (=Φܫ Φ=)∫  ฅ(＾・ω・＾ฅ)".magenta());

    Ok(())
}


