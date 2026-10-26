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

    for 

	if args.included_features().len() > 0 {
        for column in args.included_features() {
            
        }
    } else if args.excluded_features().len() > 0 {
        // excluded features arm
    } else {
        // all features arm
    }
}
