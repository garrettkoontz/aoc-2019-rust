use std::fs;

pub fn read_file(s: &str) -> Vec<String> {
    let data = fs::read_to_string(s).expect("Unable to read file");
    let mut v: Vec<String> = Vec::new();
    for s in data.trim().split('\n') {
        v.push(s.to_string());
    }
    v
}
