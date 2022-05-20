use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct BitField8 {
    pub length: usize,
    values: Vec<u8>,
}

impl BitField8 {
    pub fn new(length: usize) -> BitField8 {
        let length = (length as f64 / 8.0).ceil() as usize;
        BitField8 {
            length,
            values: vec![0; length],
        }
    }
    pub fn to_packed(&self) -> Vec<u8> {
        let mut e = ZlibEncoder::new(Vec::new(), Compression::new(6));
        e.write_all(&self.values).ok();
        e.finish().expect("flate2 should compress")
    }

    pub fn get(&self, i: usize) -> bool {
        let index = i / 8;
        let bit = i % 8;

        if index >= self.values.len() {
            false
        } else {
            (self.values[index] >> bit) & 1 != 0
        }
    }

    pub fn set(&mut self, i: usize, val: bool) {
        let index = i / 8;
        let mask = 1 << (i % 8);

        if index >= self.values.len() {
            self.values = [self.values.clone(), vec![0; index - self.values.len() + 1]].concat();
        }
        self.length = self.values.len() * 8;
        if val {
            self.values[index] |= mask;
        } else {
            self.values[index] &= !mask;
        }
    }

    pub fn last_index_of(&self, val: bool) -> Option<usize> {
        for i in (0..self.length - 1).rev() {
            if self.get(i) == val {
                return Some(i);
            }
        }
        None
    }
}

impl TryFrom<(Vec<u8>, Option<usize>)> for BitField8 {
    type Error = std::io::Error;
    fn try_from((compressed, length): (Vec<u8>, Option<usize>)) -> Result<Self, Self::Error> {
        let mut values = vec![];
        let mut decoded = ZlibDecoder::new(&compressed[..]);
        decoded.read_to_end(&mut values)?;
        let length = length.unwrap_or_else(|| values.len() * 8);
        let bytes = (length as f64 / 8.0).ceil() as usize;
        if bytes > values.len() {
            values.extend(vec![0; bytes - values.len()]);
        }
        Ok(BitField8 { length, values })
    }
}
