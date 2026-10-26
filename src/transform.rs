use std::fs::File;

use csv::Reader;

use strum::IntoStaticStr;

use crate::{
	Column,
	command::TrainArgs,
};

type Row = (String, )

pub fn filter_and_transform(
	reader: Reader<File>,
	args: TrainArgs,
) -> (Vec<Vec<f32>>, Vec<String>) {
    let matrix: Vec<Vec<f32>> = Vec::new();

    let records = reader.records();

    for row in records {
        let new_row = Vec<f32>;
        if args.included_features().len() > 0 {
            // included features arm
            for column in args.included_features() {
                
            }
        } else if args.excluded_features().len() > 0 {
            // excluded features arm
        } else {
            // all features arm
        }
    }

}

/// A struct to define how a row should look immediately after being read from file.
struct TempRow {
    
}
