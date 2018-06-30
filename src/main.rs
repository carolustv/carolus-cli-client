#[macro_use] extern crate log;
extern crate clap;
extern crate gstreamer as gst;
extern crate simplelog;

use clap::{Arg, ArgMatches, App, SubCommand};
use gst::prelude::*;
use log::LevelFilter;
use simplelog::TermLogger;

fn main() {
    let matches =
        App::new("carolus-cli")
            .version("0.1.0")
            .about("CLI client for Carolus")
            .author("Simon Dickson")
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .arg(Arg::with_name("host")
                .short("h")
                .env("CAROLUS_SERVER_URL")
                .required(true)
                .help("The url of the Carolus Server"))
            .subcommand(SubCommand::with_name("play")
                .about("plays a video in the player")
                .subcommand(SubCommand::with_name("movie")
                    .about("plays a movie")
                    .arg(Arg::with_name("title")
                        .short("t")
                        .required(true)
                        .takes_value(true)
                        .help("title of movie to play"))
                    .arg(Arg::with_name("year")
                        .short("y")
                        .takes_value(true)
                        .help("year of movie to play (only used when there are conflicts)")))
                .subcommand(SubCommand::with_name("tv")
                    .about("plays a tv episode")
                    .arg(Arg::with_name("title")
                        .short("t")
                        .required(true)
                        .takes_value(true)
                        .help("title of tv show to play"))
                    .arg(Arg::with_name("series")
                        .short("s")
                        .required(true)
                        .takes_value(true)
                        .help("series number of tv show to play"))
                    .arg(Arg::with_name("episode")
                        .short("e")
                        .required(true)
                        .takes_value(true)
                        .help("episode number of tv show to play"))
                    .arg(Arg::with_name("year")
                        .short("y")
                        .takes_value(true)
                        .help("year of movie to play (only used when there are conflicts)"))))
            .get_matches();

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
