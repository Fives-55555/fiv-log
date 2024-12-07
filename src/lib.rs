use fiv_date::custom_format_struct;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
    time::SystemTime
};

//This macro is from another crate from myself
custom_format_struct!(Date, "{DD}.{MM}.{YYYY}-{hh}:{mm}:{ss}.{fff}");

/// Use this function to log your message in the file: "last.log". It is the only thing needed.
/// If necessery you can fork or download this repository and change the name in the log_init function.
/// Maybe later I will add support for changing it with a function interface.
/// The 'info' parameter prints the level for easy finding certain errors in large log files.
/// The function is thread-safe and can be called in a parralel program, but understand
/// there could be very small drawbacks(ns) at a large scale if every thread is tring to access the MutexLock.
/// In defense I don't know of a other way to access a file handle fast or write to a file asyncly.
pub fn log<S: ToString>(info: LEVEL, msg: S) {
    let mut x = unsafe { 
        //Acquires the Mutex Lock
        #[allow(static_mut_refs)]
        LOG_FILE.lock().unwrap()
    };
    //Checks if the File Handle is initilized
    let file = match &mut *x {
        Some(file) => {
            file
        },
        None => {
            //Else initilizes the File Handle
            log_init();
            log(info, msg);
            return
        },
    };
    //Writes the message to the log file with the current date
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

//Inits the Log File Handle once
fn log_init() {
    unsafe {
        LOG_FILE = 
            //Inits a MutexLock for Thread Safety    
            Mutex::new(
                //Expresses a existing value
                Some(
                    //Gets the needed File Handle of the file:"last.log"
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

/// You should use the Debug Level only for displaing information to devs or
/// use it as optional feature to display hidden information
pub const DEBUG: LEVEL = LEVEL(INNERLEVEL::Debug);
/// You should use this Level as default to to display information that
/// are important for the logs if they are transmitted
pub const INFO: LEVEL = LEVEL(INNERLEVEL::Info);
/// You should use this Level to display error that aren't fatal but should be solved
pub const ERROR: LEVEL = LEVEL(INNERLEVEL::Error);
/// You should use this Level to display the state or relevant information of the program
/// in it's crashing state
pub const FATAL: LEVEL = LEVEL(INNERLEVEL::Fatal);
/// You should use this Level to display if a expected impossible state of the program
pub const IMPOSSIBLE: LEVEL = LEVEL(INNERLEVEL::Impossible);


static mut LOG_FILE: Mutex<Option<File>> = Mutex::new(None);

/// The struct represents the importants of a log message
pub struct LEVEL(INNERLEVEL);

// Inner Enum to represent it
enum INNERLEVEL {
    Info,
    Error,
    Fatal,
    Debug,
    Impossible,
}

impl LEVEL {
    // Function returns the &str value for the state
    fn to_string(&self) -> &str {
        match self.0 {
            INNERLEVEL::Impossible => "Impossible",
            INNERLEVEL::Info => "Info",
            INNERLEVEL::Debug => "Debug",
            INNERLEVEL::Error => "Error",
            INNERLEVEL::Fatal => "Fatal",
        }
    }
}
