# cstea
This is a generic csv file Fill and Pour Ingredient crate for use with the `rettle` ETL.

## Data Structures
- FillCsvArg: Ingredient params for FillCsTea
- FillCsTea: Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.

## Example
```rust
fn main() {
    let test_csvarg = FillCsvArg::new("fixtures/test.csv", 50);
    let brewery = Brewery::new(4, Instant::now());
    let mut new_pot = Pot::new();
    let fill_cstea = FillCsTea::new::<CsTea>("csv_tea_source", "csv_fixture", test_csvarg);

    new_pot.add_source(fill_cstea);

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
```
