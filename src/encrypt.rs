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
fn keygen(pass: &str, salt: &Salt , sens: bool) -> Key { 
    let mut kpass = Key([0u8; KEYBYTES]);
    let Key(ref mut kb) = kpass;
    let (ops, mem) = match sens { 
        true => (OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE),
        false => (OPSLIMIT_INTERACTIVE, MEMLIMIT_INTERACTIVE) ,
    }; 
    pwhash::derive_key(kb, pass.as_bytes(), &salt, ops, mem )
        .expect("keygen failed (=xܫ x=)∫");  

    return kpass;
}

#[instrument]
pub fn encrypt(in_put: &mut File, out_put: &mut File, pass: &str, sens: bool) -> Result<(), Box<dyn error::Error> >{
    
    in_put.seek(SeekFrom::Start(0))?;

    let salt = pwhash::gen_salt();
    let kpass = keygen(pass, &salt, sens);

    out_put.write(&MEOWGIC_NUM)?; 

    
    out_put.write(&salt.0)?; 

    
    let (mut stream, header) = Stream::init_push(&kpass)
        .map_err(|_| NekewError::create("\nFailed to initialize encryption stream (=xܫ x=)∫"))?; 
    
    out_put.write(&header.0)?; 

    
    let mut remainder = in_put.metadata()?.len(); 
    let mut buf = [0; CHUNK_SIZE]; 

    loop { 
        match (*in_put).read(&mut buf) { 
            Ok(num_read) if num_read > 0 => { 
                remainder -= num_read as u64; 
                let tag = match remainder { 
                    0 => Tag::Final, 
                    _ => Tag::Message,
                };
                let ebytes = &stream.push(&buf[..num_read], None, tag)
                    .map_err(|_| NekewError::create("\nFile encryption failed (=xܫ x=)∫"))?;
                out_put.write(ebytes)?;
                continue;
            } ,
            Err(e) => Err(e)?, 
            _ => break,
        }
    }
    Ok(())
} 

#[instrument]
pub fn decrypt(in_put: &mut File, out_put: &mut File, pass: &str, sens: bool)-> Result<(), Box<dyn error::Error>>{
    
    //if !(in_put.metadata()?.len() > (SALTBYTES + HEADERBYTES + MEOWGIC_NUM.len()) as u64) { 
        //return Err(NekewError::create("\nThis wasn't encrypted by this program (=xܫ x=)∫"))?;
    //}

    let mut salt = [0u8; SALTBYTES];
    in_put.read_exact(&mut salt)?; 

    let salt = Salt(salt);
    
    let mut header = [0u8; HEADERBYTES]; 
    in_put.read_exact(&mut header)?; 
    let header = Header(header); 


    let kpass = keygen(pass, &salt, sens); 
    
    let mut stream = Stream::init_pull(&header, &kpass)
        .map_err(|_| NekewError::create("\nFailed to to start encryption (=xܫ x=)∫"))?; 

    let mut buffer = [0u8; CHUNK_SIZE + ABYTES]; 

    

    while stream.is_not_finalized() { 
        match in_put.read(&mut buffer) { 
            Ok(num_read) if num_read > 0 => { 
                let (dec, _tag) = stream.pull(&buffer[..num_read], None)
                    .map_err(|_| NekewError::create("\n Wrong Password (=xܫ x=)∫"))?; 
                out_put.write(&dec)?;
                continue;
            }, 
            Err(e) => return Err(Box::new(e)), 
            _ => break,
        }
    }
    Ok(())
} 

//this is taken directly from chloride, another rust file encryption tool
//https://github.com/notdeclan/chlorine , this and cloaker basically served as documentation for sodiumoxide
pub fn verify(in_put: &mut File) -> bool {
    let fsize = in_put.metadata()
        .expect("\nFailed to retrieve metadata (=xܫ x=)∫")
        .len(); 

    if fsize < (MEOWGIC_NUM.len() + HEADERBYTES + SALTBYTES) as u64 { 
        return false;
    } 
    let mut meow = [0u8; MEOWGIC_NUM.len()]; 
    in_put.read_exact(&mut meow).expect("Failed to read magic numbers (=xܫ x=)∫"); 

    meow == MEOWGIC_NUM
}