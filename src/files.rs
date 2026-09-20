use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    let mut path = PathBuf::from(env::var("HOME").expect("HOME не найден"));

    path.push("sites.txt");

    path
}

pub fn load_sites() -> HashMap<String, String> {
    let content = fs::read_to_string(get_path()).expect("Не удалось прочитать файл");

    let mut sites = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();

        let name = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        let url = match parts.next() {
            Some(value) => value,
            None => continue,
        };

        sites.insert(name.to_string(), url.to_string());
    }

    sites
}
