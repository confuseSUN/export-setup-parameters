use ark_bn254::{Fq, Fq2, G2Affine};
use ark_ec::AffineRepr;
use ark_serialize::CanonicalSerialize;
use std::{
    fs::File,
    io::{Cursor, Write},
};
use std::io::{Read, Seek, SeekFrom};
use std::str::FromStr;
use ark_ff::PrimeField;

fn main() {
    let mut file_in = File::open("transcript00.dat").unwrap();
    file_in.seek(SeekFrom::Start(322560000u64 + 28u64)).unwrap();

    let mut g2 = Vec::<G2Affine>::new();

    let x_a0 = Fq::from_str("10857046999023057135944570762232829481370756359578518086990519993285655852781").unwrap();
    let x_a1 = Fq::from_str("11559732032986387107991004021392285783925812861821192530917403151452391805634").unwrap();
    let y_a0 = Fq::from_str("8495653923123431417604973247489272438418190587263600148770280649306958101930").unwrap();
    let y_a1 = Fq::from_str("4082367875863433681332203403145435568316851327593401208105741076214120093531").unwrap();

    let x = Fq2::new(x_a0, x_a1);
    let y = Fq2::new(y_a0, y_a1);

    g2.push(G2Affine::new(x, y));

    let mut x_a0_bytes = [0u8; 32];
    let mut x_a1_bytes = [0u8; 32];
    let mut y_a0_bytes = [0u8; 32];
    let mut y_a1_bytes = [0u8; 32];

    file_in.read(&mut x_a0_bytes).unwrap();
    file_in.read(&mut x_a1_bytes).unwrap();
    file_in.read(&mut y_a0_bytes).unwrap();
    file_in.read(&mut y_a1_bytes).unwrap();

    let mut x_a0_bigendian_bytes = [0u8; 32];
    let mut x_a1_bigendian_bytes = [0u8; 32];
    let mut y_a0_bigendian_bytes = [0u8; 32];
    let mut y_a1_bigendian_bytes = [0u8; 32];
    for j in 0..4 {
        for k in 0..8 {
            x_a0_bigendian_bytes[(4-1-j)*8+k] = x_a0_bytes[j*8+k];
            x_a1_bigendian_bytes[(4-1-j)*8+k] = x_a1_bytes[j*8+k];
            y_a0_bigendian_bytes[(4-1-j)*8+k] = y_a0_bytes[j*8+k];
            y_a1_bigendian_bytes[(4-1-j)*8+k] = y_a1_bytes[j*8+k];
        }
    }

    let x_a0 = Fq::from_be_bytes_mod_order(&x_a0_bigendian_bytes);
    let x_a1 = Fq::from_be_bytes_mod_order(&x_a1_bigendian_bytes);
    let y_a0 = Fq::from_be_bytes_mod_order(&y_a0_bigendian_bytes);
    let y_a1 = Fq::from_be_bytes_mod_order(&y_a1_bigendian_bytes);

    let x = Fq2::new(x_a0, x_a1);
    let y = Fq2::new(y_a0, y_a1);

    g2.push(G2Affine::new(x, y));

    let mut file_out = File::create("g2_2.dat").unwrap();
    let mut serialized = vec![0u8; G2Affine::generator().uncompressed_size()];

    for elem in g2.iter() {
        let mut cursor = Cursor::new(&mut serialized[..]);
        elem.serialize_uncompressed(&mut cursor).unwrap();

        file_out.write_all(&serialized[..]).unwrap();
    }
}
