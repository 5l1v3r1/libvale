use std::{error::Error, path::Path, io::Cursor};

use reqwest;
use zip_extract::{ZipExtractError};

const RELEASES: &str = "https://github.com/errata-ai/vale/releases/download";
const LATEST: &str = "https://github.com/errata-ai/vale/releases/latest";

fn get_version(mut version: String) -> Result<String, Box<dyn Error>> {
    if version == "latest" {
        let resp = reqwest::blocking::get(LATEST)?.url().to_string();
        let parts = resp.split('/');

        version = parts.last().unwrap().to_string();
        version.remove(0); // Strip the leading "v"
    }
    Ok(version)
}

pub fn install(path: &Path, version: &str, arch: &str) -> Result<(), ZipExtractError>{
    let v = get_version(version.to_string()).unwrap();

    let mut asset: String = format!("/v{}/vale_{}_{}.tar.gz", v, v, arch);
    if arch.to_lowercase().contains("windows") {
        asset = format!("/v{}/vale_{}_{}.zip", v, v, arch);
    }
    let url: String = RELEASES.to_owned() + &asset;

    let resp = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let archive: Vec<u8> = resp.to_vec();

    let buff = Cursor::new(archive);
    return zip_extract::extract(buff, path, true);
}
