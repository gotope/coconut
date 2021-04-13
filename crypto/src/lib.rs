extern crate crypto as rcrypto;
extern crate primitives;
extern crate siphasher;

use rcrypto::{digest::Digest, sha2::Sha256, sha1::Sha1, ripemd160::Ripemd160};
use siphasher::sip::SipHasher24;
use std::hash::Hasher;



