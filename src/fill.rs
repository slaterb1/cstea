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
/// Data structure that holds the Ingredient params for FillCsTea.
pub struct CsvArg {
    /// The filepath to the csv that will be processed.
    filepath: String,
    buffer_length: usize,
}

impl CsvArg {
    pub fn new(filepath: &str, buffer_length: usize) -> CsvArg {
        let filepath = String::from(filepath);
        CsvArg { filepath, buffer_length }
    }
}

impl Argument for CsvArg {
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
    pub fn new<T: Tea + Send + Debug + ?Sized + 'static>(name: &str, source: &str, params: CsvArg) -> Box<Fill> 
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
            let box_args = box_args.as_any().downcast_ref::<CsvArg>().unwrap();
            
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
