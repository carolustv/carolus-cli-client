#[macro_use] extern crate log;
extern crate clap;
extern crate gstreamer as gst;
extern crate simplelog;

use std::str::FromStr;
use std::io;

use clap::{ArgMatches, Shell};
use gst::prelude::*;
use log::LevelFilter;
use simplelog::TermLogger;

mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    match matches.subcommand() {
        ("completions", Some(matches)) => {
            let shell = Shell::from_str(matches.value_of("shell").unwrap()).unwrap();
            cli::build_cli().gen_completions_to("carolus-cli", shell, &mut io::stdout());
            return;
            }
        _ => (),
    }

    init_logging(matches.occurrences_of("v"));

    let host = matches.value_of("host").unwrap();

    match matches.subcommand() {
        ("play", Some(matches)) => {
            handle_play(host, matches);
        }
        (command, _) => error!("unhandled command: {}", command),
    }
}

fn start_player(uri: &str) {
    gst::init().unwrap();

    let pipeline = gst::parse_launch(&format!("playbin uri={}", uri)).unwrap();

    let ret = pipeline.set_state(gst::State::Playing);
    assert_ne!(ret, gst::StateChangeReturn::Failure);

    let bus = pipeline.get_bus().unwrap();
    while let Some(msg) = bus.timed_pop(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    msg.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                break;
            }
            _ => (),
        }
    }

    let ret = pipeline.set_state(gst::State::Null);
    assert_ne!(ret, gst::StateChangeReturn::Failure);
}

fn init_logging(level: u64) {
    let log_filter =
        match level {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

    TermLogger::init(log_filter, Default::default()).unwrap();
}

fn handle_play(host: &str, matches: &ArgMatches) {
    let uri =
        match matches.subcommand() {
            ("movie", Some(matches)) => {
                let year = matches.value_of("year").map_or("".to_owned(), |y|format!("?year={}", y));
                format!("{}/api/movies/play/{}{}", host, escape_string(matches.value_of("title").unwrap()), year)
            },
            ("tv", Some(matches)) => {
                let year = matches.value_of("year").map_or("".to_owned(), |y|format!("?year={}", y));
                format!("{}/api/tv/play/{}/{}/{}{}", host, escape_string(matches.value_of("title").unwrap()), matches.value_of("series").unwrap(), matches.value_of("episode").unwrap(), year)
            },
            (command, _) => panic!("unhandled command: {}", command),
        };

    start_player(&uri);
}

fn escape_string(s: &str) -> String {
    s.replace(" ", "%20")
}
