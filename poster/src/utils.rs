pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
