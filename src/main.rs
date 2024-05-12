pub mod constants;
pub mod data_format;
pub mod utils;

use std::{collections::HashMap, error::Error, fs::File};

use constants::{COLUMNS_TO_INCLUDE, DATA_FILE};
use data_format::Value;
use utils::build_dataset;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = csv::Reader::from_path(DATA_FILE)?;
    let mut dataset = build_dataset(data);
    

    let k = data
        .records()
        //.take(2)
        // take only col_to_include values
        .map(|record| {
            record
                .unwrap()
                .iter()
                .enumerate()
                .filter(|(index, _field_value)| {
                    col_to_include
                        .iter()
                        .any(|(index_to_include, _)| index == index_to_include)
                })
                .map(|(index, field_value)| {
                    {
                        let k = Value {
                            column_name: get_col_name_from_index(index, &col_to_include).unwrap(),
                            value: field_value.to_string(),
                        };
                        //println!("{k}");
                        k
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten();

    fill_dataset(&mut dataset, k);

    let k = dataset.into_iter().map(|(k,v)| Series::from_iter(v.into_iter()).with_name(&k));
    
    let mut df = DataFrame::new(k.collect()).unwrap_or_default();
    df.apply("Kg", parse_kg)?;
    //println!("{}",df.select(["Kg"])?);
    write_csv(&mut df)?;
    Ok(())
}

fn write_csv(dataset: &mut DataFrame) -> Result<(), Box<dyn Error>> {
    let mut writer = CsvWriter::new(File::create("cleared_dataset.csv")?).include_header(true);
    writer.finish(dataset)?;
    Ok(())
}

fn parse_kg(str_val: &Series) -> Series {
    str_val.str()
        .unwrap()
        .into_iter()
        .map(|opt_name: Option<&str>| {
            opt_name.map(|name: &str| name.parse().unwrap())
         })
        .collect::<UInt32Chunked>()
        .into_series()
}


fn get_col_name_from_index(
    index: usize,
    col_to_include: &HashMap<usize, String>,
) -> Option<String> {
    /* Return the column name corresponding to the given index asserted by col_to_include */
    // index is given from the StringRecord enumeration so from the order of column in the file red
    if col_to_include.contains_key(&index) {
        Some(col_to_include.get(&index).unwrap().to_owned())
    } else {
        None
    }
}

type _ColumnName = String;
fn fill_dataset<T: Iterator<Item = Value>>(
    dataset: &mut HashMap<_ColumnName, Vec<String>>,
    iterator: T,
) {
    iterator.for_each(|value| {
        dataset
            .get_mut(&value.column_name)
            .unwrap()
            .push(value.value)
    });
}
