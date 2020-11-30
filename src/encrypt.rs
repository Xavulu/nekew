#![allow(dead_code, unused_imports)]

use paris::Logger; 
use sodiumoxide::crypto::secretstream::{Stream, Tag, KEYBYTES, HEADERBYTES, ABYTES};
use sodiumoxide::crypto::secretstream::xchacha20poly1305::{Header, Key};
use sodiumoxide::crypto::pwhash::{Salt, gen_salt, SALTBYTES, MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE};
use sodiumoxide::crypto::pwhash;
use color_eyre::{Report, Result}; 




pub fn encrypt(){} 

pub fn decrypt(){} 
