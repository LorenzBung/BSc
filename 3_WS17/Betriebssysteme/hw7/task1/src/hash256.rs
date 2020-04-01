use hasher_sha256::Sha256;
use hasher_sha256::{HashResult, Hasher};
use std::fmt;

/// Dieser Datentyp beschreibt die Lösung des Hashverfahrens.
pub struct Solution {
    pub number: usize,
    pub hash: String,
}

/// Display-Implementierung für den Datentyp Solution
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number: {} --> hash: {}", self.number, self.hash)
    }
}

/// `verify_product` dient der Verifikation eines gefundenen Hashes.
pub fn verify_product(base: usize, number: usize, difficulty: &String) -> Option<Solution> {
    let sol = base * number;
    let input = sol.to_string();
    let bytes = input.as_bytes();

    let hash = Sha256::hash(bytes).hex();

    if hash.ends_with(difficulty) {
        return Some(Solution {
            number: number,
            hash: hash,
        });
    }

    None
}
