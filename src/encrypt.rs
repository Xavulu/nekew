#![allow(dead_code, unused_imports)]

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::{error, fmt};

use paris::Logger; 
use sodiumoxide::crypto::secretstream::{Stream, Tag, KEYBYTES, HEADERBYTES, ABYTES};
use sodiumoxide::crypto::secretstream::xchacha20poly1305::{Header, Key};
use sodiumoxide::crypto::pwhash::{Salt, gen_salt, SALTBYTES, MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE};
use sodiumoxide::crypto::pwhash;
use color_eyre::{Report, Result}; 
use tracing::{info, instrument};

const MEOWGIC_NUM: [u8; 4] = [0x6E, 0x65, 0x4b, 0x77];
const CHUNK_SIZE: usize = 4096;

#[derive(Debug)] 
struct NekewError { 
    message: String,
}

impl NekewError { 
    fn create(msg: &str) -> Self { 
        NekewError{
            message: msg.to_string()
        }
    }
}

impl fmt::Display for NekewError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", self.message)
    }
} 

impl error::Error for NekewError {}



#[instrument]
pub fn encrypt(in_put: &mut File, out_put: &mut File, pass: &str, sens: bool) -> Result<(), Box<dyn error::Error> >{
    
    let mut buf = [0; CHUNK_SIZE]; 
    let mut remainder = in_put.metadata()?.len(); 

    out_put.write(&MEOWGIC_NUM)?; 

    let salt = pwhash::gen_salt();
    out_put.write(&salt.0)?; 

    let mut kpass = [0u8; KEYBYTES];
    match sens { 
        true => pwhash::derive_key(&mut kpass, pass.as_bytes(), &salt, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE).unwrap(), 
        false => pwhash::derive_key(&mut kpass, pass.as_bytes(), &salt, OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE).unwrap(),
    };
    let kpass = Key(kpass); 
    let (mut stream, header) = Stream::init_push(&kpass)
        .map_err(|_| NekewError::create("\nfailed to initialize encryption stream (=xܫ x=)∫"))?; 
    out_put.write(&header.0)?; 

    loop { 
        match (*in_put).read(&mut buf) { 
            Ok(num_read) if num_read > 0 => { 
                remainder -= num_read as u64; 
                let tag = match remainder { 
                    0 => Tag::Final, 
                    _ => Tag::Message,
                };
                out_put.write(&stream.push(&buf[..num_read], None, tag)
                    .map_err(|_| NekewError::create("\nFile encryption failed (=xܫ x=)∫"))?)?;
                continue
            } ,
            Err(e) => Err(e)?, 
            _ => break,
        }
    }

    Ok(())
} 

#[instrument]
pub fn decrypt(in_put: &mut File, out_put: &mut File, pass: &str, sens: bool)-> Result<(), Box<dyn error::Error>>{
    

    print!("enc");
    Ok(())
} 

//this is taken directly from chloride, another rust file encryption tool
//https://github.com/spieglt/Cloaker , this and cloaker basically served as documentation for sodiumoxide
pub fn verify(in_put: &mut File) -> bool {
    let fsize = in_put.metadata()
        .expect("failed to retrieve metadata (=xܫ x=)∫")
        .len(); 

    if fsize < (MEOWGIC_NUM.len() + HEADERBYTES + SALTBYTES) as u64 { 
        return false;
    } 
    let mut meow = [0u8; MEOWGIC_NUM.len()]; 
    in_put.read_exact(&mut meow).expect("failed to read magic numbers (=xܫ x=)∫"); 

    meow == MEOWGIC_NUM
}