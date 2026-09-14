use std::collections::HashMap;

mod files;

fn get_connection_status(sites: HashMap<String, String>,) -> HashMap<String, String> {
    let mut results = HashMap::new();

    for (name, url) in &sites {

        let status = match reqwest::blocking::get(url) {
            Ok(response) => {
                response.status().to_string()
            }
            Err(e) => {
                format!("Ошибка: {:?}", e)
            }
        };

    results.insert(name.clone(), status);
}

    results
}

fn interface(sites: HashMap<String, String>) {
    for (name, status) in &sites {
        println!("{} -> {}", name, status);
    }
}


fn main() {

    let sites = files::load_sites();

    // get list of site
    // move it in array or something
    // via loop get array of site name andresponses like name and key 
    //
    // let url = env::args()
    //     .nth(1)
    //     .expect("Укажите URL");
    //
    // let status = get_connection_status(&url);
    

    let site_n_status = get_connection_status(sites);
    interface(site_n_status);
    
}
