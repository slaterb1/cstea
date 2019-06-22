extern crate cstea;
extern crate rettle;
extern crate serde;

use cstea::fill::{CsvArg, fill_from_csv};
use rettle::tea::Tea;
use rettle::brewer::Brewery;
use rettle::pot::Pot;
use rettle::ingredient::{Fill, Pour};

use std::any::Any;
use std::time::Instant;
use serde::Deserialize;

#[derive(Default, Clone, Debug, Deserialize)]
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
            fill_from_csv::<CsTea>(args, brewery, recipe);
        }),
        params: Some(Box::new(test_csvarg))
    }));

    new_pot.add_ingredient(Box::new(Pour{
        name: String::from("pour1"),
        computation: Box::new(|tea_batch, _args| {
            tea_batch.into_iter()
                .map(|tea| {
                    println!("Final Tea: {:?}", tea.as_any().downcast_ref::<CsTea>().unwrap());
                    let tea = tea.as_any().downcast_ref::<CsTea>().unwrap();
                    let same_tea = tea.clone();
                    Box::new(same_tea) as Box<dyn Tea + Send>
                })
                .collect()
        }),
        params: None,
    }));

    new_pot.brew(&brewery);
}
