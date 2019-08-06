# cstea

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.com/slaterb1/cstea.svg?branch=master)](https://travis-ci.com/slaterb1/cstea)
[![Crates.io Version](https://img.shields.io/crates/v/cstea.svg)](https://crates.io/crates/cstea)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.35.0+-lightgray.svg)](#rust-version-requirements)

This is a generic csv file Fill and Pour Ingredient crate for use with the `rettle` ETL.

## Data Structures
- FillCsvArg: Ingredient params for FillCsTea
- FillCsTea: Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.
- PourCsvArg: Ingredient params for PourCsTea
- PourCsTea: Wrapper to simplifiy the creation of the Pour Ingredient to be used in the rettle Pot.

## Example
```rust
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

fn main() {
    let test_csvarg = FillCsvArg::new("fixtures/test.csv", 50);
    let test_pour_csvarg = PourCsvArg::new("fixtures/pour.csv");

    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();
    let fill_cstea = FillCsTea::new::<CsTea>("csv_tea_source", "csv_fixture", test_csvarg);
    let pour_cstea = PourCsTea::new::<CsTea>("csv_pour_test", test_pour_csvarg);

    new_pot.add_source(fill_cstea);

    // Steep operations of choice

    new_pot.add_ingredient(pour_cstea);

    new_pot.brew(&brewery);
}
```
