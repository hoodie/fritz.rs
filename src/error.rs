use reqwest;
use serde_xml_rs;

use std::io;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    links {
    }

    foreign_links {
        Reqwest(reqwest::Error);
        Url(reqwest::UrlError);
        Xml(serde_xml_rs::Error);
        Io(io::Error);
    }

    errors {
    }
}
