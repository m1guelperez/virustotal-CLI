use std::thread::sleep;
use std::time::Duration;
use virustotal::{FileScanResponse, UrlScanResponse, VtClient};
use crate::request_controller::MAXIMUM_REQUESTS_PER_MINUTE;

pub struct ResponseControllerUrl {
    api_key: String,
    response_code: i32,
    scan_id: Option<String>,
    scan_date: Option<String>,
    verbose_msg: String,
    url: Option<String>,
    permalink: Option<String>,
}

impl ResponseControllerUrl {
    pub fn new(key: String, response: UrlScanResponse) -> Self {
        ResponseControllerUrl {
            api_key: key,
            response_code: response.response_code,
            scan_id: response.scan_id,
            scan_date: response.scan_date,
            verbose_msg: response.verbose_msg,
            url: response.url,
            permalink: response.permalink,
        }
    }

    ///Maybe change this to one url only instead of Vector?
    pub fn analyze_url_report(&self) -> (String, i32) {
        let client = VtClient::new(self.api_key.as_str());
        //default value if an error occurs
        let mut scan_results = ("Error occurred".to_string(), -999);
        match &self.scan_id {
            Some(v) => {
                println!("Sending report request for: {}", &self.url.as_ref().unwrap());
                let positives = client.report_url(v).positives.unwrap() as i32;
                scan_results = (self.url.as_ref().unwrap().clone(), positives);
            }
            None => (),
        }

        scan_results
    }
}

pub struct ResponseControllerFile {
    api_key: String,
    file_name: String,
    response_code: i32,
    verbose_msg: String,
    sha_256: Option<String>,
    scan_id: Option<String>,
    permalink: Option<String>,
    resource: Option<String>,
}

impl ResponseControllerFile {
    pub fn new(key: String, name: &String, response: FileScanResponse) -> Self {
        ResponseControllerFile {
            api_key: key,
            file_name: name.to_string(),
            response_code: response.response_code,
            resource: response.resource,
            permalink: response.permalink,
            verbose_msg: response.verbose_msg,
            sha_256: response.sha256,
            scan_id: response.scan_id,
        }
    }

    pub fn analyze_file_report(&self) -> (String, i32) {
        let client = VtClient::new(self.api_key.as_str());
        let mut scan_results = ("Error occurred".to_string(), -999);

        //Try at most 2 times before canceling (API limit)
        let mut retries = 0;
        while retries < MAXIMUM_REQUESTS_PER_MINUTE {
            match &self.sha_256 {
                Some(v) => {
                    let temp_results = client.report_file(v.as_str());
                    if temp_results.response_code == 1 {
                        println!("Sending report request for: {}.", self.file_name);
                        scan_results = (self.file_name.clone(), temp_results.positives.unwrap() as i32);
                        return scan_results;
                    } else {
                        retries += 1;
                        println!("Retrying to get report for {}", self.file_name);
                        //Sleep timer is needed because the scan results aren't available instantly depending on the file size.
                        sleep(Duration::new(30, 0))
                    }
                }
                None => (),
            }
        }
        scan_results
    }
}