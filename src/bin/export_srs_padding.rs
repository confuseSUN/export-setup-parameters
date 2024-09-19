use std::{
    fs::File,
    io::Write,
};

mod kzg;
use kzg::KZGCommitmentScheme;

fn main() {
    let bytes = std::fs::read("srs.dat").expect("NO SRS FILE");
    let scheme = KZGCommitmentScheme::from_unchecked_bytes(&bytes);

    let mut new_g1 = Vec::with_capacity(2057);

    new_g1.extend_from_slice(&scheme.public_parameter_group_1[0..2051]);
    new_g1.extend_from_slice(&scheme.public_parameter_group_1[4096..4099]);
    new_g1.extend_from_slice(&scheme.public_parameter_group_1[8192..8195]);

    // 3. create crs.
    let crs = KZGCommitmentScheme {
        public_parameter_group_1: new_g1,
        public_parameter_group_2: scheme.public_parameter_group_2,
    };
    println!("srs success!");

    // 4. srs serialize to file.
    let mut file_out = File::create("srs-padding.bin").unwrap();
    let bytes = crs.to_unchecked_bytes();
    file_out.write_all(&bytes).unwrap();

    println!("serialize to file: srs-padding.bin");
}
