use fiv_date::custom_format_struct;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
    time::SystemTime
};

custom_format_struct!(Date, "{DD}.{MM}.{YYYY}-{hh}:{mm}:{ss}.{fff}");

pub fn log<S: ToString>(info: LEVEL, msg: S) {
    let mut x = unsafe { LOG_FILE.lock().unwrap() };
    let file = match x.as_mut() {
        Some(file) => {
            file
        },
        None => {
            log_init();
            log(info, msg);
            return;
        },
    };
    match file.write_all(
        format!(
            "\n{}-{}; {}",
            info.to_string(),
            Date::now(&SystemTime::now()),
            msg.to_string()
        )
        .as_bytes(),
    ) {
        Ok(_) => (),
        Err(_) => panic!("Fatal Error at writing to Log File"),
    }
}

fn log_init() {
    unsafe {
        LOG_FILE = Mutex::new(
            Some(
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("last.log")
                    .unwrap()
            )
        );
    }
}   

pub const IMPOSSIBLE: LEVEL = LEVEL(INNERLEVEL::Impossible);
pub const INFO: LEVEL = LEVEL(INNERLEVEL::Info);
pub const DEBUG: LEVEL = LEVEL(INNERLEVEL::Debug);
pub const ERROR: LEVEL = LEVEL(INNERLEVEL::Error);
pub const FATAL: LEVEL = LEVEL(INNERLEVEL::Fatal);

static mut LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

pub struct LEVEL(INNERLEVEL);

enum INNERLEVEL {
    Info,
    Error,
    Fatal,
    Debug,
    Impossible,
}

impl LEVEL {
    fn to_string(&self) -> String {
        match self.0 {
            INNERLEVEL::Impossible => "Impossible",
            INNERLEVEL::Info => "Info",
            INNERLEVEL::Debug => "Debug",
            INNERLEVEL::Error => "Error",
            INNERLEVEL::Fatal => "Fatal",
        }
        .to_string()
    }
}
