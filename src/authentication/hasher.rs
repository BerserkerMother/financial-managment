use sha2::Digest;
use sha2::Sha256;
use std::string::String;

pub trait Hash {
    fn hash(self) -> String;
}

impl Hash for String {
    fn hash(self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.as_bytes());
        let mut password = hasher.finalize();

        String::from_utf8_lossy(password.as_mut_slice()).into_owned()
    }
}
