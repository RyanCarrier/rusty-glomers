use crate::message::MaelstromMessage;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use state::State;

use std::io::{self, BufRead};

mod message;
mod state;

fn main() {
    let mut state: State = State::new();
    input_loop(&mut state);
}

fn input_loop(state: &mut State) {
    let stdin: io::Stdin = io::stdin();
    let mut lines_stream = stdin.lock().lines();
    loop {
        match lines_stream.next() {
            Some(Ok(valid_input)) => {
                let msg: MaelstromMessage = serde_json::from_str(&valid_input).unwrap();
                state.handle(msg);
            }
            Some(Err(_)) => {}
            None => {}
        }
    }
}

pub fn setup_logging(pre: Option<String>) {
    let location = String::from("/home/rcarrier/Projects/rusty-glomers/rusty-glomers.log");
    let mut pattern = String::from("{l} - {m}\n");
    match pre {
        Some(f) => pattern = format!("{{l}} - [{}] {{m}}\n", f),
        None => {}
    }
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(&pattern)))
        .build(location)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
}
