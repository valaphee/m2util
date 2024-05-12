use std::{ffi::CStr, path::{Path, PathBuf}};

use crate::crypt;

pub struct Pack {
    indices: Vec<Index>,
    content: Vec<u8>,
}

impl Pack {
    pub fn open(index_path: impl AsRef<Path>, content_path: impl AsRef<Path>) -> Self {
        let index_data = crypt::read(&std::fs::read(index_path).unwrap(), &ETERPACK_INDEX_KEY);
        let index_magic = &index_data[0..4];
        if index_magic != b"EPKD" {
            todo!()
        }

        let index_version = u32::from_le_bytes(index_data[4..8].try_into().unwrap()) as usize;
        if index_version != 2 {
            todo!()
        }

        let index_count = u32::from_le_bytes(index_data[8..12].try_into().unwrap()) as usize;
        println!("{}", index_count);
        Self {
            indices: unsafe { std::slice::from_raw_parts(index_data[12..].as_ptr() as *const _, index_count)}.to_owned(),
            content: std::fs::read(content_path.as_ref().to_owned()).unwrap(),
        }
    }

    pub fn indices(&self) -> &[Index] {
        &self.indices
    }

    pub fn get(&self, id: usize) -> Option<Vec<u8>> {
        let index = self.indices.get(id as usize)?;
        let data = &self.content[index.offset as usize..][..index.encoded_size as usize + 4];
        Some(crypt::read(data, &ETERPACK_CONTENT_KEY))
    }
}

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct Index {
    id: u32,
    filename: [u8; 161],
    _pad: [u8; 3],
    filename_crc: u32,
    content_size: u32,
    encoded_size: u32,
    crc: u32,
    offset: u32,
    encoding: u8,
    _pad2: [u8; 3],
}

impl Index {
    pub fn id(&self) -> usize {
        self.id as usize
    }

    pub fn filename(&self) -> &str {
        CStr::from_bytes_until_nul(&self.filename).unwrap().to_str().unwrap()
    }
}

const ETERPACK_INDEX_KEY: [u8; 16] = [0xB9, 0x9E, 0xB0, 0x02, 0x6F, 0x69, 0x81, 0x05, 0x63, 0x98, 0x9B, 0x28, 0x79, 0x18, 0x1A, 0x00];
const ETERPACK_CONTENT_KEY: [u8; 16] = [0x22, 0xB8, 0xB4, 0x04, 0x64, 0xB2, 0x6E, 0x1F, 0xAE, 0xEA, 0x18, 0x00, 0xA6, 0xF6, 0xFB, 0x1C];
