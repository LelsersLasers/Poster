use crate::*;


pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(password.as_bytes());
    let hashed = hasher.finish();
    hashed.to_string()
}
