use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use virustotal::{FileScanResponse, UrlScanResponse, VtClient};
use crate::RequestControllerClient;

pub struct ResponseControllerUrl {
    api_key: String,
    response_code: i32,
    scan_id: Option<String>,
    scan_date: Option<String>,
    verbose_msg: String,
    url: Option<String>,
    permalink: Option<String>,
}


pub struct ResponseControllerFile {
    api_key: String,
    response_code: i32,
    verbose_msg: String,
    sha_256: Option<String>,
    scan_id: Option<String>,
    permalink: Option<String>,
    resource: Option<String>,
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
                let positives = client.report_url(v).positives.unwrap() as i32;
                scan_results = (self.url.as_ref().unwrap().clone(), positives);
            }
            None => (),
        }

        scan_results
    }
}

impl ResponseControllerFile {
    pub fn new(key: String, response: FileScanResponse) -> Self {
        ResponseControllerFile {
            api_key: key,
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
        let mut maximum_retries = 0;

        while maximum_retries < 4 {
            match &self.sha_256 {
                Some(v) => {
                    let temp_results = client.report_file(v.as_str());
                    if temp_results.response_code == 1 {
                        scan_results = (self.resource.as_ref().unwrap().clone(), temp_results.positives.unwrap() as i32);
                        return scan_results;
                    } else {
                        maximum_retries += 1;
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