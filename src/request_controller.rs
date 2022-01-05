use std::fs;
use virustotal::VtClient;
use crate::response_controller::{ResponseControllerFile, ResponseControllerUrl};

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
    pub fn send_url_scan(&self, url: &str) -> Vec<ResponseControllerUrl> {
        let scan_client = VtClient::new(self.api_key.as_str());
        let scan_result = scan_client.scan_url(url);

        let vec_queue = vec![ResponseControllerUrl::new(self.api_key.to_string(), scan_result)];

        vec_queue
    }

    ///At most 4 files, because the free API only allows 4 scans per minute.
    /// This methods send 4 file scans from a given folder.
    pub fn send_file_scans(&self, path: String) -> Vec<ResponseControllerFile> {
        let files = fs::read_dir(path).unwrap();
        let client = VtClient::new(&self.api_key);

        let mut vec_results = Vec::new();

        for file in files.take(4) {
            match client.scan_file(file.unwrap().path().display().to_string().as_str()) {
                Ok(v) => vec_results.push(ResponseControllerFile::new(self.api_key.to_string(), v)),
                Err(_e) => continue,
            };
        }
        vec_results
    }
}

