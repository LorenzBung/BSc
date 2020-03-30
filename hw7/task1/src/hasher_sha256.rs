extern crate sha2;
#[cfg(feature = "SHA2")]
use self::sha2::Sha256;


pub struct Sha256;

pub trait Hasher {
    type Output: HashResult;
    fn hash(input: &[u8]) -> Self::Output;
}

pub trait HashResult {
    /// Get the output in hex notation.
    fn hex(&self) -> String;
    /// Size of the output in bytes.
    fn size() -> usize;
}


impl Hasher for Sha256 {
    type Output = [u8; 32];

    fn hash(input: &[u8]) -> Self::Output {
        use self::sha2::*;
        let mut tmp = Sha256::new();
        tmp.input(input);
        let r = tmp.result().as_slice().to_vec();
        [
            r[0],
            r[1],
            r[2],
            r[3],
            r[4],
            r[5],
            r[6],
            r[7],
            r[8],
            r[9],
            r[10],
            r[11],
            r[12],
            r[13],
            r[14],
            r[15],
            r[16],
            r[17],
            r[18],
            r[19],
            r[20],
            r[21],
            r[22],
            r[23],
            r[24],
            r[25],
            r[26],
            r[27],
            r[28],
            r[29],
            r[30],
            r[31],
        ]
    }
}

impl HashResult for [u8; 32] {
    fn hex(&self) -> String {
        const HEX: [char; 16] = [
            '0',
            '1',
            '2',
            '3',
            '4',
            '5',
            '6',
            '7',
            '8',
            '9',
            'a',
            'b',
            'c',
            'd',
            'e',
            'f',
        ];
        let mut tmp = String::with_capacity(32 * 2);
        for byte in self.iter() {
            tmp.push(HEX[*byte as usize / 16]);
            tmp.push(HEX[*byte as usize % 16]);
        }
        tmp
    }

    fn size() -> usize {
        32
    }
}


#[test]
fn test_hash() {
    assert_eq!(
        Sha256::hash("test".as_bytes()).hex(),
        "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
    );
}
