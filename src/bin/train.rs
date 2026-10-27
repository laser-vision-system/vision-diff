use std::{
	error::Error,
	path::Path,
	process,
};

use csv::Reader;
use vision_diff::{
	CSVRowItem,
	Column,
	config::read_config,
	path::resolve_path,
	transform::CSVMatrix,
};

fn main() -> Result<(), Box<dyn Error>> {
	run()
}

fn run() -> Result<(), Box<dyn Error>> {
	let config = read_config()?;
	let mut csv_reader = Reader::from_path(config.general().input_data_dir())?;
	// let mut reader_iter = csv_reader.deserialize();
	let mut matrix: CSVMatrix = CSVMatrix(Vec::new());

	for result in csv_reader.deserialize() {
		let row: CSVRowItem = result?;
		matrix.0.push(row);
	}

	let (matrix, included_features) = matrix.transform(&config)?;

	

	

	Ok(())
}
