//! Wrapper around `Vec<u8>`

use std::{ops, str, fmt, io, marker};
use hex::{ToHex, FromHex, FromHexError};
use heapsize::HeapSizeOf;

/// Wrapper around `Vec<u8>`
#[derive(Default, PartialEq, Clone, Eq, Hash)]
pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new() -> Self {
        Bytes::default()
    }
    pub fn new_with_len(len: usize) -> Self {
        Bytes(vec![0; len])
    }
    pub fn take(self) -> Vec<u8> {
        self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn append(&mut self, ot: &mut Bytes) {
        self.0.append(&mut ot.0)
    }
    pub fn split_off(&mut self, at:usize) -> Bytes {
        Bytes(self.0.split_off(at))
    }

}
