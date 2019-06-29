/*!
# cstea
This is a generic csv file Fill and Pour Ingredient crate for use with the `rettle` ETL.

## Data Structures
- FillCsvArg: Ingredient params for FillCsTea
- FillCsTea: Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.
- PourCsvArg: Ingredient params for PourCsTea
- PourCsTea: Wrapper to simplifiy the creation of the Pour Ingredient to be used in the rettle Pot.

## Example
```rust
fn main() {
    let test_csvarg = FillCsvArg::new("fixtures/test.csv", 50);
    let test_pour_csvarg = PourCsvArg::new("fixtures/pour.csv");

    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();
    let fill_cstea = FillCsTea::new::<CsTea>("csv_tea_source", "csv_fixture", test_csvarg);
    let pour_cstea = PourCsTea::new::<CsTea>("csv_pour_test", test_pour_csvarg);

    new_pot.add_source(fill_cstea);

    // Steep operatoins of choice

    new_pot.add_ingredient(pour_cstea);

    new_pot.brew(&brewery);
}
```
*/

pub mod fill;
pub mod pour;
