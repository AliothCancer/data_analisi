use std::ops::Not;

use csv::StringRecord;

use crate::constants::COLUMNS_TO_INCLUDE;

pub fn build_dataset(data: &StringRecord) -> [(String,Vec<String>);10] {

    let col_to_include = data.iter()
        .enumerate()
        .filter(|(index, column_name_from_data)| {
            COLUMNS_TO_INCLUDE.iter().any(|column_to_include| {
                column_to_include.to_lowercase() == column_name_from_data.to_lowercase() || ([22 as usize,19 as usize].contains(index)).not() 
                // there are couples for column called "ECG" index 22 and "sintomi" index 19
                // so im removing to have consistent lengh otherwise ->
                // this error will occur
                // CSV error: found record with 12 fields, but the previous record has 10 fields
            })
        })
        .map(|(x, y)| (x, y.to_string()));


    let mut dataset: [(String,Vec<String>);10];
    dataset.iter_mut().zip(col_to_include).for_each(|((col_name_tomut, val_tomut),(_col_index,col_name))|{
        *col_name_tomut = col_name.to_owned();
    });

    dataset
}
