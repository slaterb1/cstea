extern crate rettle;
extern crate csv;
extern crate serde;

use rettle::ingredient::{Ingredient, Argument};
use rettle::brewer::{Brewery};
use rettle::tea::Tea;

use std::sync::{Arc, RwLock};
use std::io::{BufReader};
use std::fs::File;
use std::any::Any;
use serde::Deserialize;
use std::fmt::Debug;

pub struct CsvArg {
    pub filepath: String
}

impl Argument for CsvArg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn fill_from_csv<T: Tea + Debug + Send + ?Sized>(args: &Option<Box<dyn Argument + Send>>, brewery: &Brewery, recipe: Arc<RwLock<Vec<Box<dyn Ingredient + Send + Sync>>>>, tea_struct: Box<T>) 
where
    for<'de> T: Deserialize<'de>
{
    match args {
        None => panic!("Need to pass \"filepath\" param!"),
        Some(box_args) => {
            let box_args = box_args.as_any().downcast_ref::<CsvArg>().unwrap();
            let f = File::open(&box_args.filepath).unwrap();
            let reader = BufReader::new(f);
            let mut rdr = csv::Reader::from_reader(reader);
            
            //let mut tea_box: Vec<T> = vec![];
            for result in rdr.deserialize() {
                let record: T = result.unwrap();
                println!("{:?}", record);
                //tea_box.push(record);
            }
        }
    }
}
