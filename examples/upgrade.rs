use std::path::PathBuf;

fn main() {
    let data = std::fs::read("/Users/gngpp/VSCode/ninja/tLfeYLzFNDDz-YFDI_84l.json").unwrap();
    let target: Vec<String> = serde_json::from_slice(&data).unwrap();
    let host = "https://cdn.oaistatic.com";
    let client = reqwest::blocking::ClientBuilder::new()
        .impersonate(reqwest::impersonate::Impersonate::OkHttpAndroid13)
        .timeout(std::time::Duration::from_secs(60))
        .connect_timeout(std::time::Duration::from_secs(30))
        .cookie_store(true)
        .build()
        .unwrap();

    for path in target {
        let req_url = format!("{}/{}", host, path);
        let resp = client.get(&req_url).send().unwrap();
        let file_path = PathBuf::from("upgrade").join(path);
        if file_path.exists() {
            continue;
        }
        if let Some(p) = file_path.parent() {
            if !p.exists() {
                std::fs::create_dir_all(p).unwrap();
            }
        }
        if resp.status().is_success() {
            std::fs::write(file_path, resp.bytes().unwrap()).unwrap();
            println!("downloaded: {}", req_url);
        } else {
            panic!("request failed: {}", req_url);
        }
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
