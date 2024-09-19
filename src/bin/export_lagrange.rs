use ark_bn254::{Fr, G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use std::{fs::File, io::Write};

mod kzg;
use kzg::KZGCommitmentScheme;

fn main() {
    let n = std::env::args()
        .nth(1)
        .expect("missing n powers (Degree 2^n, 10 <= n <= 21)");
    let n: u32 = n.parse().expect("invalid number n");
    let degree = 2usize.pow(n);

    println!("load srs file...");
    let bytes = std::fs::read("srs.dat").expect("NO SRS FILE");
    let p = KZGCommitmentScheme::from_unchecked_bytes(&bytes);
    let p1: Vec<G1Projective> = p.public_parameter_group_1[..degree]
        .iter()
        .map(|v| v.into_group())
        .collect();

    println!("start ifft...");
    let domain = Radix2EvaluationDomain::<Fr>::new(degree).unwrap();
    let lagrange_p1: Vec<G1Affine> = domain.ifft(&p1).iter().map(|v| v.into_affine()).collect();
    println!("over ifft...");

    let new_p = KZGCommitmentScheme {
        public_parameter_group_1: lagrange_p1,
        public_parameter_group_2: vec![],
    };

    let mut file_out = File::create(format!("lagrange-srs-{}.bin", degree)).unwrap();
    let bytes = new_p.to_unchecked_bytes();
    file_out.write_all(&bytes).unwrap();

    println!("serialize to file: lagrange-srs-{}.bin", degree);
}
