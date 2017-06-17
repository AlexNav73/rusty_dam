
use crypto::Keccak;

pub mod record;
pub mod file;
pub mod field;
pub mod field_group;
pub mod classification;
pub mod user;
pub mod session;
pub mod collections;
pub mod pg;
pub mod es;

fn get_sha3<S: AsRef<str>>(text: S) -> String {
    let mut sha3 = Keccak::new_sha3_256();
    sha3.update(text.as_ref().as_bytes());
    let mut res = [0; 32];
    sha3.finalize(&mut res);

    String::from_utf8_lossy(&res).into_owned()
}

