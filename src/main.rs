extern crate gstreamer as gst;
use gst::prelude::*;
use std::env;

fn main() {
    gst::init().unwrap();

    let args = env::args().collect::<Vec<_>>();
    let uri = format!("http://192.168.1.3:8000/api/movies/play/{}", args[1]);
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
