[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Check and Lint](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/check-and-lint.yaml/badge.svg)](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/check-and-lint.yaml)
[![Tests](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/test.yaml/badge.svg)](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/test.yaml)
[![Release](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/release-packaging.yaml/badge.svg)](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/release-packaging.yaml)
[![Rust](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/rust.yml/badge.svg)](https://github.com/m1guelperez/VirusTotal-Folderscanner/actions/workflows/rust.yml)

## About The Project
This application is a CLI application for VirusTotal written in Rust.

You will need a valid API key to use it, which you can get at (https://www.virustotal.com/gui/join-us).

The free API has a limit of 4 requests per minute and 500 requests per day.
Therefore, this program will scan only the first 4 files in a folder, during default mode. 

Open the program via the commandline and provide the api key in the configfile and a list of URLs you want to scan.

## Caution!
The `testWebsites.txt` is purely for testing. Do NOT click on any of those links. 

### Built with
* [Rust]

## Getting started
Example how to run the program yourself:

### Prerequisites
Fill in the API_KEY in a configfile.txt.

## Running the programming:
You can run the program with the following command:

* [powershell]
    ````sh
    VirusTotal_Folderscanner [API_KEY], [URLs]

### Installation:

1. Get a free API Key at [https://www.virustotal.com/gui/join-us](https://www.virustotal.com/gui/join-us)
2. Clone the repo 
    ````sh
   git clone https://github.com/m1guelperez/VirusTotal-Folderscanner.git
3. Build the program yourself.
4. Go to the folder, containing the .exe and see Prerequisites above.

## Roadmap

- [x] Added pre-release version with default values
- [x] Create option to read API_KEY configfile
  - [ ] Read args from commandline
    - [x] Url
    - [ ] Filepaths
- [ ] Release version 1.0
- [ ] Support Linux filesystem
- [ ] Option to read paths from configfile
- [ ] Add max_scans feature (use max. free API request limit.)
- [ ] Add GUI

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

## Contact

Miguel Perez - [@m1guelperez](https://twitter.com/m1guelperez) 

Email: - perezdevelopmentofficial[at]gmail.com

Project Link: [https://github.com/m1guelperez/VirusTotal-Folderscanner](https://github.com/m1guelperez/VirusTotal-Folderscanner)

## Acknowledgments

Thanks to [@owlinux1000](https://github.com/owlinux1000) for providing great [rust-bindings](https://github.com/owlinux1000/virustotal.rs) 
to use the virustotal-api in Rust.