#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;

use serde_xml_rs::from_str;
use serde_xml_rs::to_string;
use log::*;

fn init_logger() {
    use log::{LogLevel, LogMetadata, LogRecord};

    struct SimpleLogger;

    impl log::Log for SimpleLogger {
        fn enabled(&self, metadata: &LogMetadata) -> bool {
            metadata.level() <= LogLevel::Debug
        }

        fn log(&self, record: &LogRecord) {
            if self.enabled(record.metadata()) {
                println!("{} - {}", record.level(), record.args());
            }
        }
    }

    let _ = log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::Debug);
        Box::new(SimpleLogger)
    });
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetRelaties {
    #[serde(rename = "SessionID")]
    pub session_id: Option<String>,
    #[serde(rename = "SecurityCode2")]
    pub security_code_2: Option<String>,
    #[serde(rename = "cFilter")]
    pub c_filter: Option<cRelatieFilter>,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct cRelatieFilter {
    #[serde(rename = "Trefwoord")]
    pub trefwoord: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "ID")]
    pub id: i64,
}

fn main() {
    init_logger();

    info!("test");

    let gr = GetRelaties {
        session_id: Some("session_id".to_string()),
        security_code_2: Some("security_code_2".to_string()),
        c_filter: Some(cRelatieFilter {
            trefwoord : None,
            code: None,
            id: 13786118,
        }),
    };

    println!("{}", to_string(&gr).unwrap());

}