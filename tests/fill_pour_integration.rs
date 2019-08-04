extern crate cstea;
extern crate rettle;
extern crate serde;

use cstea::fill::{FillCsvArg, FillCsTea};
use cstea::pour::{PourCsvArg, PourCsTea};
use rettle::tea::Tea;
use rettle::brewer::Brewery;
use rettle::pot::Pot;

use std::any::Any;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::fs;
use std::thread;
use std::time::Duration;
use std::io::BufReader;
use std::path::Path;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
struct CsTea {
    id: i32,
    name: String,
    value: i32
}

impl Tea for CsTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_fill_pour() {
    if Path::new("fixtures/test.csv").exists() {
        fs::remove_file("fixtures/test.csv").unwrap();
    }

    let test_fill_csvarg = FillCsvArg::new("fixtures/fill.csv", 50);
    let test_pour_csvarg = PourCsvArg::new("fixtures/test.csv");

    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();
    let fill_cstea = FillCsTea::new::<CsTea>("csv_tea_source", "csv_fixture", test_fill_csvarg);
    let pour_cstea = PourCsTea::new::<CsTea>("csv_pour_test", test_pour_csvarg);

    new_pot.add_source(fill_cstea);
    new_pot.add_ingredient(pour_cstea);
    new_pot.brew(&brewery);

    thread::sleep(Duration::from_millis(400));

    let input_file = fs::File::open("fixtures/fill.csv").unwrap();
    let output_file = fs::File::open("fixtures/test.csv").unwrap();

    let in_reader = BufReader::new(input_file);
    let out_reader = BufReader::new(output_file);

    let mut in_rdr = csv::Reader::from_reader(in_reader);
    let mut out_rdr = csv::Reader::from_reader(out_reader);

    let mut counter1 = 0;
    let mut counter2 = 0;
    for _result in in_rdr.records() {
        counter1 += 1;
    }

    for _result in out_rdr.records() {
        counter2 += 1;
    }

    assert_eq!(counter1, counter2);
    assert_ne!(counter1, 0);
    assert_ne!(counter2, 0);

    fs::remove_file("fixtures/test.csv").unwrap();
}

