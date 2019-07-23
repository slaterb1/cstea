use rettle::ingredient::{ Argument, Pour};
use rettle::tea::Tea;

use std::any::Any;
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::Path;

///
/// Ingredient params for PourCsTea.
pub struct PourCsvArg {
    /// The filepath to the csv that will be processed.
    filepath: String,
}

impl PourCsvArg {
    ///
    /// Returns a PourCsvArg to be used as params in PourCsTea.
    ///
    /// # Arguments
    ///
    /// * `filepath` - filepath for csv to load.
    pub fn new(filepath: &str) -> PourCsvArg {
        let filepath = String::from(filepath);
        PourCsvArg { filepath }
    }
}

impl Argument for PourCsvArg {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

///
/// Wrapper to simplifiy the creation of the Pour Ingredient to be used in the rettle Pot.
pub struct PourCsTea {}

impl PourCsTea {
    ///
    /// Returns the Pour Ingredient to be added to the `rettle` Pot.
    ///
    /// # Arguments
    ///
    /// * `name` - Ingredient name
    /// * `params` - Params data structure holding the `filepath` for the csv to process
    pub fn new<T: Tea + Send + Sync + Clone + Serialize + 'static>(name: &str, params: PourCsvArg) -> Box<Pour> {
        Box::new(Pour {
            name: String::from(name),
            computation: Box::new(|tea_batch, args| {
                pour_to_csv::<T>(tea_batch, args)
            }),
            params: Some(Box::new(params))
        })
    }
}

///
/// Implements the csv pour, with serialization based on specified data struct.
/// brewery for processing.
///
/// # Arguments
///
/// * `tea_batch` - Vec<Box<dyn Tea + Send>> batch that needs to be output to csv
/// * `args` - Params specifying the filepath of the csv.
fn pour_to_csv<T: Tea + Send + Sync + Clone + Serialize + 'static>(tea_batch: Vec<Box<dyn Tea + Send>>, args: &Option<Box<dyn Argument + Send>>) -> Vec<Box<dyn Tea + Send>> {
    match args {
        None => panic!("Need to pass \"filepath\" params!"),
        Some(box_args) => {
            // unwrap params
            let box_args = box_args.as_any().downcast_ref::<PourCsvArg>().unwrap();
            // TODO: update to use from_writer and pass Writer with append and remove headers
            // (check existence first for headers)
            //let headers: bool;
            let headers = !Path::new(&box_args.filepath).exists();
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(&box_args.filepath)
                .unwrap();

            let mut writer = csv::WriterBuilder::new().has_headers(headers).from_writer(file);

            tea_batch.into_iter()
                .map(|tea| {
                    let tea = tea.as_any().downcast_ref::<T>().unwrap();
                    let same_tea = tea.clone();
                    writer.serialize(&same_tea).unwrap();

                    Box::new(same_tea) as Box<dyn Tea + Send>
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PourCsvArg, PourCsTea};
    use rettle::tea::Tea;
    use rettle::pot::Pot;
    use serde::Serialize;
    use std::any::Any;

    #[derive(Default, Clone, Debug, Serialize)]
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
        let csv_args = PourCsvArg::new("fixtures/test.csv");
        assert_eq!(csv_args.filepath, "fixtures/test.csv");
    }

    #[test]
    fn create_pour_cstea() {
        let csv_args = PourCsvArg::new("fixtures/test.csv");
        let pour_cstea = PourCsTea::new::<TestCsTea>("test_csv", csv_args);
        let new_pot = Pot::new();
        new_pot.add_ingredient(pour_cstea);
        assert_eq!(new_pot.get_recipe().read().unwrap().len(), 1);
        assert_eq!(new_pot.get_recipe().read().unwrap()[0].get_name(), "test_csv");
    }
}

