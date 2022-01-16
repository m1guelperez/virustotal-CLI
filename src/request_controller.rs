use std::fs;
use virustotal::VtClient;
use crate::response_controller::{ResponseControllerFile, ResponseControllerUrl};

pub const MAXIMUM_REQUESTS_PER_MINUTE: i8 = 2;

pub struct RequestControllerClient {
    api_key: String,
}

impl RequestControllerClient {
    pub fn new(key: &str) -> RequestControllerClient {
        RequestControllerClient {
            api_key: key.to_string(),
        }
    }

    ///At most 4 url scans, because of the free API limit
    pub fn send_url_scan(&self, urls: Vec<String>) -> Vec<ResponseControllerUrl> {
        let scan_client = VtClient::new(self.api_key.as_str());
        let mut vec_queue = Vec::new();
        for url in urls.iter().take(MAXIMUM_REQUESTS_PER_MINUTE as usize) {
            println!("Sending url scan request for: {}", url);
            let scan = scan_client.scan_url(url);
            vec_queue.push(ResponseControllerUrl::new(self.api_key.to_string(), scan));
        };
        vec_queue
    }

    ///At most 2 files, because the free API only allows 2 scans per minute.
    /// This method sends 2 file scans from a given folder.
    pub fn send_file_scans(&self, path: String) -> Vec<ResponseControllerFile> {
        let files = fs::read_dir(&path).unwrap();
        let client = VtClient::new(&self.api_key);
        let mut vec_queue = Vec::new();
        for file in files.take(MAXIMUM_REQUESTS_PER_MINUTE as usize) {
            println!("Send file scan request for: {}", &file.as_ref().unwrap().path().display().to_string());
            match client.scan_file(file.as_ref().unwrap().path().display().to_string().as_str()) {
                Ok(v) => vec_queue.push(ResponseControllerFile::new(self.api_key.to_string(), &file.unwrap().path().display().to_string(), v)),
                Err(_e) => continue,
            };
        }
        vec_queue
    }
}

