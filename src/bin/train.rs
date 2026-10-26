use std::{
	error::Error,
	path::Path,
	process,
};

use vision_diff::{
	command::get_train_args,
	path::resolve_path,
};

use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
	run()
}

fn run() -> Result<(), Box<dyn Error>> {
	let args = get_train_args();
	let csv_path: Path = resolve_path(args.data())?.into();

    let csv_reader= Reader::from_path(csv_path)?;

    

	Ok(())
}
