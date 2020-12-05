#![allow(dead_code, unused_imports)]

use std::path::Path;
use std::fs::File;

use paris::Logger; 
use sodiumoxide::crypto::secretstream::{Stream, Tag, KEYBYTES, HEADERBYTES, ABYTES};
use sodiumoxide::crypto::secretstream::xchacha20poly1305::{Header, Key};
use sodiumoxide::crypto::pwhash::{Salt, gen_salt, SALTBYTES, MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE};
use sodiumoxide::crypto::pwhash;
use color_eyre::{Report, Result}; 
use tracing::{info, instrument};

const MEOWGIC_NUM: [u8; 4] = [0x6E, 0x65, 0x4b, 0x77];
const CHUNK_SIZE: usize = 4096;

pub fn encrypt(in_put: &mut File, out_put: &mut File, pass: &str) -> Result<(), Report>{
    let mut logger = Logger::new();
    logger.loading("<blue>beginning file encryption</>  <magenta>ฅ(＾・ω・＾ฅ)</>");


    Ok(())
} 

pub fn decrypt(in_put: &mut File, out_put: &mut File, pass: &str)-> Result<(), Report>{
    let mut logger = Logger::new();
    logger.loading("<blue>beginning file decryption</>  <magenta>ฅ(＾・ω・＾ฅ)</>");


    Ok(())
} 
