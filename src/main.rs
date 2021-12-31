use std::collections::HashMap;
use crate::request_controller::{RequestControllerClient};

mod response_controller;
mod request_controller;

fn main() {
    //TODO: ApiKey per args and eventually default key
    let client = RequestControllerClient::new("");
    let res = client.send_url_scan("http://myetherevvalliet.com/");

    let mut all_scan_results = HashMap::new();

    for re in res {
        let vec_url_scan_result = re.analyze_url_report();
        all_scan_results.insert(vec_url_scan_result.0,vec_url_scan_result.1);
    }

    println!("{:?}",all_scan_results);
}
