extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;

fn abort(s: String ){
    println!("{}", s);
    process::exit(1);
}

fn main() {
    let mut seed = false;
    let mut num_queues = 3;
    let mut quantum = 10;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut seed)
            .add_option(&["-s", "--seed"], StoreTrue,
            "the random seed");
        ap.refer(&mut num_queues)
            .add_option(&["-n", "--numQueues"], Store,
            "number of queues in MLFQ (if not using -Q)");
        ap.refer(&mut quantum)
            .add_option(&["-q", "--quantum"], Store,
            "length of time slice (if not using -Q)");
        ap.parse_args_or_exit();
    }
}
