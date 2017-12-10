extern crate chrono;
extern crate crypto;
extern crate encode_unicode;
#[macro_use]
extern crate error_chain;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use  encode_unicode::CharExt;

use crypto::digest::Digest;
use crypto::md5::Md5;
use reqwest::{Url, Client};

use std::io::Read;
use std::ops::Deref;

mod error;

use error::*;

#[derive(Debug)]
struct SID(pub String);

impl Deref for SID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl From<String> for SID {
    fn from(sid_str: String) -> SID {
        SID(sid_str)
    }
}


fn execute_cmd(cmd: &str, sid: SID) -> String {
    let host = "http://192.168.1.227/webservices/homeautoswitch.lua";
    let ain = "5C:49:79:F8:E1:34";

    let url = Url::parse_with_params(
        host,
        &[("ain", ain),
          ("switchcmd", cmd),
          ("sid", &sid)
         ]).unwrap();

    let client = Client::new();
    let mut res = client.get(url)
        .send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    return content.trim().into();
}

#[derive(Debug, Deserialize)]
struct SessionInfo {
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "Challenge")]
    pub challenge: String,
    #[serde(rename = "BlockTime")]
    pub block_time: usize,
    //Rights: {}

}

fn to_utf16_bytes(input: &str) -> Vec<u8> {
    input.chars()
         .flat_map(|c| c.iter_utf16_units())
         .map(|word| vec![
             word as u8,
             (word >> 8) as u8,
         ])
         .flat_map(|pair| pair.into_iter())
         .collect()
}

fn generate_challenge_response(password: &str, challenge: &str) -> String {
    let response = format!("{}-{}", challenge, password);
    let response_bytes = to_utf16_bytes(&response);

    let mut digest = Md5::new();
    digest.input(&response_bytes);
    format!("{}-{}", challenge, digest.result_str())
}

fn send_challenge_response(response: &str) -> Result<SessionInfo> {
    let response_url = Url::parse_with_params(
        "http://fritz.box/login_sid.lua",
        &[("response", &response)]
    )?;

    let mut content = String::new();
    reqwest::get(response_url)?.read_to_string(&mut content)?;

    let session_info: SessionInfo = serde_xml_rs::deserialize(content.as_bytes())?;
    Ok(session_info)
}

fn login(password: &str) -> Result<SessionInfo> {
    let challenge_url = Url::parse("http://fritz.box/login_sid.lua")?;

    let mut content = String::new();
    reqwest::get(challenge_url)?.read_to_string(&mut content)?;

    let session_info: SessionInfo = serde_xml_rs::deserialize(content.as_bytes())?;
    let response = generate_challenge_response(&password, &session_info.challenge);
    send_challenge_response(&response)
}


fn main() {
    let sid = login("****").unwrap();
    println!("{:?}", sid);

}
