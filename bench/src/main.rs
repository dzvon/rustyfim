use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    time,
};

use clap::Parser;
use rustyfim::NEclatClosed;

#[derive(Parser)]
#[clap(about = "A simple tool to get the time spent on NEclatClosed algorithm.")]
struct Opt {
    /// The path to the file containing the dataset.
    #[clap(value_parser, value_name = "FILE")]
    dataset: PathBuf,

    #[clap(short, long)]
    min_support: f32,
}

fn main() {
    let opt = Opt::parse();

    let dataset = File::open(&opt.dataset).unwrap();
    let reader = BufReader::new(dataset);

    let mut transactions = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            transactions.push(line.iter().map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    let neclat = NEclatClosed::default();

    let start = time::Instant::now();
    neclat.process(&transactions, opt.min_support);
    println!("Time spent: {:?}", start.elapsed());
}
