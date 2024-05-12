use xtea::{cipher::{array::Array, BlockCipherDecrypt, KeyInit}, Xtea};

pub fn read(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let magic = &data[0..4];
    if magic != b"MCOZ" && magic != b"MCSP" {
        todo!();
    }

    let encrypted_len = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    let compressed_len = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;
    let len = u32::from_le_bytes(data[12..16].try_into().unwrap()) as usize;

    let mut decrypted_data = data[16..][..encrypted_len + 4].to_vec();
    decrypted_data.extend_from_slice(&[0, 0, 0, 0]);
    Xtea::new_from_slice(key).unwrap().decrypt_blocks(Array::cast_slice_from_core_mut(unsafe { decrypted_data.as_chunks_unchecked_mut() }));
    let decrypted_magic = &decrypted_data[0..4];
    if decrypted_magic != magic {
        todo!();
    }

    let mut decompressed_data = Vec::with_capacity(len);
    unsafe { decompressed_data.set_len(len) };
    if magic == b"MCSP" {
        snap::raw::Decoder::new().decompress(&decrypted_data[4..][..compressed_len], &mut decompressed_data).unwrap();
    } else {
        todo!();
    }
    decompressed_data
}
