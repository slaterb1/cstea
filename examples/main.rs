use cstea::fill::{FillCsvArg, FillCsTea};
use cstea::pour::{PourCsvArg, PourCsTea};
use rettle::{
    Brewery,
    Pot,
    Argument,
    Steep,
};

use std::any::Any;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
struct CsTea {
    id: i32,
    name: String,
    value: i32
}

pub struct SteepArgs {
    pub increment: i32,
}

impl Argument for SteepArgs {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let test_fill_csvarg = FillCsvArg::new("fixtures/fill.csv", 50);
    let test_pour_csvarg = PourCsvArg::new("fixtures/pour.csv");
    let steep_args = SteepArgs { increment: 10000 };

    let brewery = Brewery::new(4);
    let mut new_pot = Pot::new();
    let fill_cstea = FillCsTea::new::<CsTea>("csv_tea_source", "csv_fixture", test_fill_csvarg);
    let pour_cstea = PourCsTea::new::<CsTea>("csv_pour_test", test_pour_csvarg);

    new_pot = new_pot.add_source(fill_cstea);

    // Add ingredients to pot
    new_pot = new_pot.add_ingredient(Box::new(Steep{
        name: String::from("steep1"),
        computation: Box::new(|tea_batch: Vec<CsTea>, args| {
            tea_batch
                .into_iter()
                .map(|mut tea| {
                    match args {
                        None => panic!("No params passed, not editing object!"),
                        Some(box_args) => {
                            let box_args = box_args.as_any().downcast_ref::<SteepArgs>().unwrap();
                            tea.value = tea.value - box_args.increment;
                        }
                    }
                    tea
                })
                .collect()
        }),
        params: Some(Box::new(steep_args)),
    }));

    new_pot = new_pot.add_ingredient(pour_cstea);

    new_pot.brew(&brewery);
}
