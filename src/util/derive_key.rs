use num_bigint::{BigUint, ToBigUint};
use sha1::Sha1;

const id: u8 = 1;
const u: usize = 160;
const v: usize = 512;

const r: usize = 2;
const n: usize = 256;

pub(crate) fn derive_key(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let mut password = password
        .iter()
        .map(|b| [0, b.clone()])
        .collect::<Vec<_>>()
        .concat();
    password.append(&mut vec![0; 2]);
    let D = vec![id; v / 8];
    let s = salt.len();
    let t = v * (s as f32 / v as f32).ceil() as usize / 8;
    let S = &vec![salt; (t as f32 / s as f32).ceil() as usize].concat()[0..t];
    let p = password.len();
    let t = v * (p as f32 / v as f32).ceil() as usize / 8;
    let P = &vec![password; (t as f32 / p as f32).ceil() as usize].concat()[0..t];
    let mut I = [S, P].concat();
    let c = (n as f32 / u as f32).ceil() as usize;
    let mut A = Vec::new();
    for _ in 0..c {
        let mut sha1 = Sha1::new();
        sha1.update(&D);
        sha1.update(&I);
        let mut Ai = sha1.digest();
        for _ in 1..r {
            let mut sha1 = Sha1::new();
            sha1.update(&Ai.bytes());
            Ai = sha1.digest();
        }
        A.push(Ai.bytes());
        let b = v / 8;
        let B = &vec![Ai.bytes(); (v as f32 / u as f32).ceil() as usize].concat()[0..b];
        let modulus = 2.to_biguint().unwrap().pow(v as u32);
        I = I
            .chunks(b)
            .map(|I_j| {
                let I_j = BigUint::from_bytes_be(I_j);
                let B = BigUint::from_bytes_be(B);
                let one = 1.to_biguint().unwrap();
                ((I_j + B + one) % modulus.clone()).to_bytes_be()
            })
            .collect::<Vec<_>>()
            .concat();
    }
    A.concat()[0..(n / 8)].to_vec()
}
