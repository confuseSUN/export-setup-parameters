use ark_bn254::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};

/// KZG commitment scheme style.
pub struct KZGCommitmentScheme {
    pub public_parameter_group_1: Vec<G1Affine>,
    pub public_parameter_group_2: Vec<G2Affine>,
}

impl KZGCommitmentScheme {
    /// serialize the parameters to unchecked bytes.
    #[allow(dead_code)]
    pub fn to_unchecked_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let len_1 = self.public_parameter_group_1.len() as u32;
        let len_2 = self.public_parameter_group_2.len() as u32;
        bytes.extend(len_1.to_le_bytes());
        bytes.extend(len_2.to_le_bytes());

        for i in &self.public_parameter_group_1 {
            i.serialize_with_mode(&mut bytes, Compress::No).unwrap();
        }
        for i in &self.public_parameter_group_2 {
            i.serialize_with_mode(&mut bytes, Compress::No).unwrap();
        }
        bytes
    }

    #[allow(dead_code)]
    pub fn from_unchecked_bytes(bytes: &[u8]) -> Self {
        let mut len_1_bytes = [0u8; 4];
        let mut len_2_bytes = [0u8; 4];
        len_1_bytes.copy_from_slice(&bytes[0..4]);
        len_2_bytes.copy_from_slice(&bytes[4..8]);
        let len_1 = u32::from_le_bytes(len_1_bytes) as usize;
        let len_2 = u32::from_le_bytes(len_2_bytes) as usize;
        let n_1 = G1Affine::default().uncompressed_size();
        let n_2 = G2Affine::default().uncompressed_size();

        let bytes_1 = &bytes[8..];
        let bytes_2 = &bytes[8 + (n_1 * len_1)..];
        let mut p1 = vec![];
        let mut p2 = vec![];

        for i in 0..len_1 {
            p1.push(
                G1Affine::deserialize_with_mode(
                    &bytes_1[n_1 * i..n_1 * (i + 1)],
                    Compress::No,
                    Validate::No,
                )
                .unwrap(),
            );
        }

        for i in 0..len_2 {
            p2.push(
                G2Affine::deserialize_with_mode(
                    &bytes_2[n_2 * i..n_2 * (i + 1)],
                    Compress::No,
                    Validate::No,
                )
                .unwrap(),
            );
        }

        Self {
            public_parameter_group_1: p1,
            public_parameter_group_2: p2,
        }
    }
}
