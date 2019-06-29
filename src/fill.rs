extern crate rettle;
extern crate csv;
extern crate serde;

use rettle::ingredient::{Ingredient, Argument, Fill};
use rettle::brewer::{Brewery, make_tea};
use rettle::tea::Tea;

use std::sync::{Arc, RwLock};
use std::io::{BufReader};
use std::fs::File;
use std::any::Any;
use serde::Deserialize;
use std::fmt::Debug;

///
/// Ingredient params for FillCsTea.
pub struct FillCsvArg {
    /// The filepath to the csv that will be processed.
    filepath: String,
    buffer_length: usize,
}

impl FillCsvArg {
    ///
    /// Returns a FillCsvArg to be used as params in FillCsTea.
    ///
    /// # Arguments
    ///
    /// * `filepath` - filepath for csv to load.
    /// * `buffer_length` - number of csv lines to process at a time.
    pub fn new(filepath: &str, buffer_length: usize) -> FillCsvArg {
        let filepath = String::from(filepath);
        FillCsvArg { filepath, buffer_length }
    }
}

impl Argument for FillCsvArg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

///
/// Wrapper to simplifiy the creation of the Fill Ingredient to be used in the rettle Pot.
pub struct FillCsTea {}

impl FillCsTea {
    ///
    /// Returns the Fill Ingredient to be added to the `rettle` Pot.
    ///
    /// # Arguments
    ///
    /// * `name` - Ingredient name
    /// * `source` - Ingredient source
    /// * `params` - Params data structure holding the `filepath` for the csv to process
    pub fn new<T: Tea + Send + Debug + ?Sized + 'static>(name: &str, source: &str, params: FillCsvArg) -> Box<Fill> 
        where for<'de> T: Deserialize<'de>
    {
        Box::new(Fill {
            name: String::from(name),
            source: String::from(source),
            computation: Box::new(|args, brewery, recipe| {
                fill_from_csv::<T>(args, brewery, recipe);
            }),
            params: Some(Box::new(params))
        })
    }
}

/// Helper function that sends to batch request to Brewers for processing.
///
/// # Arguments
///
/// * `brewery` - Brewery that processes the data.
/// * `recipe` - Recipe for the ETL used by the Brewery.
/// * `tea_batch` - Current batch to be sent and processed
fn call_brewery(brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>, tea_batch: Vec<Box<dyn Tea + Send>>) {
    brewery.take_order(|| {
        make_tea(tea_batch, recipe);
    });
}

///
/// Implements the csv read, deserialization to specified data struct, and passes the data to the
/// brewery for processing.
///
/// # Arguments
///
/// * `args` - Params specifying the filepath of the csv.
/// * `brewery` - Brewery that processes the data.
/// * `recipe` - Recipe for the ETL used by the Brewery.
fn fill_from_csv<T: Tea + Send + Debug + ?Sized + 'static>(args: &Option<Box<dyn Argument + Send>>, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>) 
    where for<'de> T: Deserialize<'de>
{
    match args {
        None => panic!("Need to pass \"filepath\" and buffer_length params!"),
        Some(box_args) => {
            // unwrap params
            let box_args = box_args.as_any().downcast_ref::<FillCsvArg>().unwrap();
            
            // initialize reader with specified file from path
            let f = File::open(&box_args.filepath).unwrap();
            let reader = BufReader::new(f);
            let mut rdr = csv::Reader::from_reader(reader);
            
            // iterate over csv lines and push data into processer
            let mut tea_batch: Vec<Box<dyn Tea + Send>> = Vec::with_capacity(box_args.buffer_length);
            for result in rdr.deserialize() {
                // check if batch size has been reached and send to brewers if so
                if tea_batch.len() == box_args.buffer_length {
                    let recipe = Arc::clone(&recipe);
                    call_brewery(brewery, recipe, tea_batch);
                    tea_batch = Vec::with_capacity(box_args.buffer_length);
                }
                let tea: T = result.unwrap();
                tea_batch.push(Box::new(tea));
            }
            let recipe = Arc::clone(&recipe);
            call_brewery(brewery, recipe, tea_batch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FillCsvArg, FillCsTea};
    use rettle::tea::Tea;
    use rettle::pot::Pot;
    use serde::Deserialize;
    use std::any::Any;

    #[derive(Default, Clone, Debug, Deserialize)]
    struct TestCsTea {
        id: i32,
        name: String,
        value: i32
    }

    impl Tea for TestCsTea {
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn new(self: Box<Self>) -> Box<dyn Tea + Send> {
            self
        }
    }

    #[test]
    fn create_csv_args() {
        let csv_args = FillCsvArg::new("fixtures/test.csv", 50);
        assert_eq!(csv_args.filepath, "fixtures/test.csv");
        assert_eq!(csv_args.buffer_length, 50);
    }

    #[test]
    fn create_fill_cstea() {
        let csv_args = FillCsvArg::new("fixtures/test.csv", 50);
        let fill_cstea = FillCsTea::new::<TestCsTea>("test_csv", "fixture", csv_args);
        let mut new_pot = Pot::new();
        new_pot.add_source(fill_cstea);
        assert_eq!(new_pot.get_sources().len(), 1);
        assert_eq!(new_pot.get_sources()[0].get_name(), "test_csv");
    }
}
