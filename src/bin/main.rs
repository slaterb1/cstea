extern crate cstea;
extern crate rettle;
extern crate serde;

use cstea::fill::{CsvArg, fill_from_csv};
use rettle::tea::Tea;
use rettle::brewer::Brewery;
use rettle::pot::Pot;
use rettle::ingredient::Fill;

use std::any::Any;
use std::time::Instant;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
struct CsTea {
    id: i32,
    name: String,
    value: i32
}

impl Tea for CsTea {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new(self: Box<Self>) -> Box<dyn Tea + Send> {
        self
    }
}

fn main() {
    let test_csvarg = CsvArg { filepath: String::from("fixtures/test.csv") };
    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();

    new_pot.add_source(Box::new(Fill{
        name: String::from("csv_tea_source"),
        source: String::from("csv_fixture"),
        computation: Box::new(|args, brewery, recipe| {
            let sample_tea = CsTea::new(Box::new(CsTea::default()));
            fill_from_csv(args, brewery, recipe, sample_tea);
        }),
        params: Some(Box::new(test_csvarg))
    }));

    new_pot.brew(&brewery);
}
