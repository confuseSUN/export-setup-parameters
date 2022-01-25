use ark_bls12_381::{Fq, Fq2, G1Affine, G2Affine};
use ark_serialize::CanonicalSerialize;
use num_bigint::BigUint;
use serde::{Deserialize, Serialize};
use std::env::args;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};
use text_io::scan;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use algebra::{
    bls12_381::{BLSG1, BLSG2},
    groups::Group,
};

/// KZG commitment scheme style.
#[derive(Serialize, Deserialize)]
struct KZGCommitmentScheme {
    public_parameter_group_1: Vec<BLSG1>,
    public_parameter_group_2: Vec<BLSG2>,
}

fn main() {
    let n = args()
        .nth(1)
        .expect("missing n powers (Degree 2^n, 10 <= n <= 21)");
    let n: u32 = n.parse().expect("invalid number n");

    // 1. create g1.
    let file_in = File::open("g1_coeffs.dat").unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g1 = Vec::<G1Affine>::new();

    let mut counter = 0;
    loop {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_str: String;
        let y_str: String;

        scan!(line.bytes() => "{} {}", x_str, y_str);

        assert!(x_str.starts_with("0x"));
        assert!(y_str.starts_with("0x"));

        let x = BigUint::parse_bytes(&x_str.as_bytes()[2..], 16).unwrap();
        let y = BigUint::parse_bytes(&y_str.as_bytes()[2..], 16).unwrap();

        let x_field_elem: Fq = x.into();
        let y_field_elem: Fq = y.into();

        g1.push(G1Affine::new(x_field_elem, y_field_elem, false));
        counter = counter + 1;
        if counter % 10000 == 0 {
            println!("{}", counter);
        }

        line.clear();
    }

    let size = 1 << n;
    println!("n: {}, size: {}", n, size);
    let mut public_parameter_group_1: Vec<BLSG1> = vec![];
    for elem in g1.iter().take(size + 3) {
        let mut serialized = Vec::new();
        elem.serialize(&mut serialized).unwrap();
        public_parameter_group_1.push(BLSG1::from_compressed_bytes(&serialized[..]).unwrap());
    }
    println!("g1 params success!");

    // 2. create g2.
    let file_in = File::open("g2_coeffs.dat").unwrap();
    let mut reader = BufReader::new(file_in);

    let mut line = String::new();

    let mut g2 = Vec::<G2Affine>::new();

    for _ in 0..2 {
        let _ = reader.read_line(&mut line).unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let x_c0_str: String;
        let x_c1_str: String;
        let y_c0_str: String;
        let y_c1_str: String;

        scan!(line.bytes() => "{} {} {} {}", x_c0_str, x_c1_str, y_c0_str, y_c1_str);

        assert!(x_c0_str.starts_with("0x"));
        assert!(x_c1_str.starts_with("0x"));
        assert!(y_c0_str.starts_with("0x"));
        assert!(y_c1_str.starts_with("0x"));

        let x_c0 = BigUint::parse_bytes(&x_c0_str.as_bytes()[2..], 16).unwrap();
        let x_c1 = BigUint::parse_bytes(&x_c1_str.as_bytes()[2..], 16).unwrap();

        let y_c0 = BigUint::parse_bytes(&y_c0_str.as_bytes()[2..], 16).unwrap();
        let y_c1 = BigUint::parse_bytes(&y_c1_str.as_bytes()[2..], 16).unwrap();

        let x_c0_field_elem: Fq = x_c0.into();
        let x_c1_field_elem: Fq = x_c1.into();

        let y_c0_field_elem: Fq = y_c0.into();
        let y_c1_field_elem: Fq = y_c1.into();

        let x_field_elem = Fq2::new(x_c0_field_elem, x_c1_field_elem);
        let y_field_elem = Fq2::new(y_c0_field_elem, y_c1_field_elem);

        let elem = G2Affine::new(x_field_elem, y_field_elem, false);
        assert!(elem.is_on_curve());
        g2.push(elem);

        line.clear();
    }

    let public_parameter_group_2: Vec<BLSG2> = ark_std::cfg_into_iter!(g2)
        .map(|elem| {
            let mut serialized = Vec::new();
            elem.serialize(&mut serialized).unwrap();
            BLSG2::from_compressed_bytes(&serialized[..]).unwrap()
        })
        .collect();
    println!("g2 params success!");

    // 3. create crs.
    let crs = KZGCommitmentScheme {
        public_parameter_group_1,
        public_parameter_group_2,
    };
    println!("crs success!");

    // 4. crs serialize to file.
    let mut file_out = File::create(format!("zei_crs_bls12381_2_{}.dat", n)).unwrap();
    file_out
        .write_all(&bincode::serialize(&crs).unwrap())
        .unwrap();

    println!("serialize to file: zei_crs_bls12381_2_{}.dat", n);
}
