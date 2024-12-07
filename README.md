# Logging Crate for Rust

## Thread-Safe minimal logging crate
This is a small crate, which has only a few features.

## How to use
'''
    use fiv_log::{INFO, DEBUG, FATAL, ERROR, log};

    fn main() {
        log(INFO, "Hey this program is running. :-)");
        let one = 1+1;
        log(DEBUG, &format!("one is {}", one));
        if one != 1 {
            log(FATAL, "WFT. Why is 1+1 not 1");
            panic!()
        }else {
            log(ERROR, "In this world is a flaw.");
        }
        while true {};
        log(IMPOSSSIBLE, "???The while true loop stoped???")
    }
'''
