use fiv_date::Date;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Once,
};

pub fn log<S: ToString>(info: LEVEL, msg: S) {
    INIT.call_once(log_init);
    let file = get_log();
    match file.write_all(
        format!(
            "\n{}-{}; {}",
            info.to_string(),
            Date::new().now(),
            msg.to_string()
        )
        .as_bytes(),
    ) {
        Ok(_) => (),
        Err(_) => panic!("Fatal Error at writing to Log File"),
    }
}

fn get_log() -> &'static mut File {
    unsafe { LOG_FILE.as_mut().unwrap() }
}

fn log_init() {
    unsafe {
        LOG_FILE = Some(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("last.log")
                .unwrap(),
        );
    }
}   

pub const IMPOSSIBLE: LEVEL = LEVEL(INNERLEVEL::Impossible);
pub const INFO: LEVEL = LEVEL(INNERLEVEL::Info);
pub const DEBUG: LEVEL = LEVEL(INNERLEVEL::Debug);
pub const ERROR: LEVEL = LEVEL(INNERLEVEL::Error);
pub const FATAL: LEVEL = LEVEL(INNERLEVEL::Fatal);

static INIT: Once = Once::new();
static mut LOG_FILE: Option<File> = None;

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
