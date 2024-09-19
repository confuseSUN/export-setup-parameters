use ark_bn254::{G1Affine, G2Affine};
use std::{
    fs::File,
    io::Write,
};
use ark_serialize::{CanonicalDeserialize, Compress, Validate};

mod kzg;
use kzg::KZGCommitmentScheme;

fn main() {
    // 1. create g1.
    let g1_bytes = std::fs::read("g1_8195.dat").expect("NO SRS FILE");
    let mut g1 = Vec::<G1Affine>::new();

    for i in 0..8195 {
        g1.push(G1Affine::deserialize_with_mode(
            &g1_bytes[64 * i..64*(i+1)],
    Compress::No,
            Validate::Yes
        ).unwrap());
    }

    let g2_bytes = std::fs::read("g2_2.dat").expect("NO SRS FILE");
    let mut g2 = Vec::<G2Affine>::new();

    for i in 0..2 {
        g2.push(G2Affine::deserialize_with_mode(
            &g2_bytes[128 * i..128*(i+1)],
            Compress::No,
            Validate::Yes
        ).unwrap());
    }

    // 3. create crs.
    let crs = KZGCommitmentScheme {
        public_parameter_group_1: g1,
        public_parameter_group_2: g2,
    };
    println!("srs success!");

    // 4. srs serialize to file.
    let mut file_out = File::create("srs.dat").unwrap();
    let bytes = crs.to_unchecked_bytes();
    file_out.write_all(&bytes).unwrap();

    println!("serialize to file: srs.dat");
}
