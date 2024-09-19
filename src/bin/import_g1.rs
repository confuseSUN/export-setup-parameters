use ark_bn254::{Fq, G1Affine};
use ark_ec::AffineRepr;
use ark_serialize::CanonicalSerialize;
use std::{
    fs::File,
    io::{Cursor, Write},
};
use std::io::{Read, Seek, SeekFrom};
use ark_ff::PrimeField;

const MAX_SIZE: usize = 8192 + 3;

fn main() {
    let mut file_in = File::open("transcript00.dat").unwrap();
    file_in.seek(SeekFrom::Start(28u64)).unwrap();

    let mut g1 = Vec::<G1Affine>::new();

    g1.push(G1Affine::new(Fq::from(1u64), Fq::from(2u64)));

    for _ in 1..MAX_SIZE {
        let mut x_bytes = [0u8; 32];
        let mut y_bytes = [0u8; 32];

        file_in.read(&mut x_bytes).unwrap();
        file_in.read(&mut y_bytes).unwrap();

        let mut x_bigendian_bytes = [0u8; 32];
        let mut y_bigendian_bytes = [0u8; 32];
        for j in 0..4 {
            for k in 0..8 {
                x_bigendian_bytes[(4-1-j)*8+k] = x_bytes[j*8+k];
                y_bigendian_bytes[(4-1-j)*8+k] = y_bytes[j*8+k];
            }
        }

        let x = Fq::from_be_bytes_mod_order(&x_bigendian_bytes);
        let y = Fq::from_be_bytes_mod_order(&y_bigendian_bytes);

        let point = G1Affine::new(x, y);
        g1.push(point);
    }

    let mut file_out = File::create("g1_8195.dat").unwrap();

    let mut serialized =
        vec![0u8; G1Affine::generator().uncompressed_size()];

    for elem in g1.iter().take(8195) {
        let mut cursor = Cursor::new(&mut serialized[..]);
        elem.serialize_uncompressed(&mut cursor).unwrap();
        file_out.write_all(&serialized[..]).unwrap();
    }
}
