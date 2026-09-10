fn main() {
    let url = "https://youtube.com";

    match reqwest::blocking::get(url) {
        Ok(response) => {
            println!("HTTP доступ есть: {}", response.status());
        }
        Err(e) => {
            println!("HTTP доступа нет: {}", e);
        }
    }
}
