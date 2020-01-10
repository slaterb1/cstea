use rettle::{
    Ingredient, 
    Argument,
    Fill,
    Brewery,
    make_tea,
};

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
    batch_size: usize,
}

impl FillCsvArg {
    ///
    /// Returns a FillCsvArg to be used as params in FillCsTea.
    ///
    /// # Arguments
    ///
    /// * `filepath` - filepath for csv to load.
    /// * `batch_size` - number of csv lines to process at a time.
    pub fn new(filepath: &str, batch_size: usize) -> FillCsvArg {
        let filepath = String::from(filepath);
        FillCsvArg { filepath, batch_size }
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
    pub fn new<T: Send + Debug + ?Sized + 'static>(name: &str, source: &str, params: FillCsvArg) -> Box<Fill<T>> 
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
fn call_brewery<T: Send + 'static>(brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>>, tea_batch: Vec<T>) {
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
fn fill_from_csv<T: Send + Debug + ?Sized + 'static>(args: &Option<Box<dyn Argument + Send>>, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient<T> + Send + Sync>>>>) 
    where for<'de> T: Deserialize<'de>
{
    match args {
        None => (),
        Some(box_args) => {
            // Unwrap params.
            let box_args = box_args.as_any().downcast_ref::<FillCsvArg>().unwrap();
            
            // Initialize reader with specified file from path.
            let f = File::open(&box_args.filepath);

            let mut rdr = match f {
                Ok(f) => {
                    let reader = BufReader::new(f);
                    csv::Reader::from_reader(reader)
                },
                Err(e) => {
                    println!("Failed opening file! Error: {:?}", e);
                    return
                },
            };
            
            // Iterate over csv lines and push data into processer
            let mut tea_batch: Vec<T> = Vec::with_capacity(box_args.batch_size);
            for result in rdr.deserialize() {
                // Check if batch size has been reached and send to brewers if so.
                if tea_batch.len() == box_args.batch_size {
                    let recipe = Arc::clone(&recipe);
                    call_brewery(brewery, recipe, tea_batch);
                    tea_batch = Vec::with_capacity(box_args.batch_size);
                }
                let tea: T = result.unwrap();
                tea_batch.push(tea);
            }
            let recipe = Arc::clone(&recipe);
            call_brewery(brewery, recipe, tea_batch);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FillCsvArg, FillCsTea};
    use rettle::{
        Tea,
        Pot,
    };
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
    }

    #[test]
    fn create_csv_args() {
        let csv_args = FillCsvArg::new("fixtures/test.csv", 50);
        assert_eq!(csv_args.filepath, "fixtures/test.csv");
        assert_eq!(csv_args.batch_size, 50);
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
