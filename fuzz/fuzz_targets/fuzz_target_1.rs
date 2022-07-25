#![no_main]

use libfuzzer_sys::fuzz_target;
use rustyfim::NEclatClosed;

fuzz_target!(|transactions: Vec<Vec<usize>>| {
    // fuzzed code goes here
    let neclat = NEclatClosed::default();
    neclat.process(&transactions, 0.5);
});
