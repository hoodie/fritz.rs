extern crate chrono;
extern crate reqwest;

use chrono::prelude::*;
use reqwest::{Url, Client};

use std::fs::File;
use std::io::{Read, Write};
use std::{thread, time};


fn execute_cmd(cmd: &str) -> String {
    let host = "http://192.168.1.227/webservices/homeautoswitch.lua";
    let ain = "5C:49:79:F8:E1:34";
    let sid = "53689bcb2c236faa";

    let url = Url::parse_with_params(
        host,
        &[("ain", ain),
          ("switchcmd", cmd),
          ("sid", sid)
         ]).unwrap();

    let client = Client::new().unwrap();
    let mut res = client.get(url).unwrap()
        .send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    return content.trim().into();
}

fn main() {

    // init log file
    let start_time = Utc::now().format("%FT%H%M%S");
    let log_file = format!("powerlog_{}.log", start_time);
    let mut buffer = File::create(&log_file).unwrap();

    loop {
        let content = execute_cmd("getswitchpower");

        let time = Utc::now().to_rfc3339();

        let log_line = format!("{} {}\n", time, content.trim());

        buffer.write(log_line.as_bytes()).unwrap();
        print!("{}", log_line);

        thread::sleep(time::Duration::from_millis(500));
    }
}
