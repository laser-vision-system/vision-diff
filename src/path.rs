use std::{
	error::Error,
	io::{
		Error as IoError,
		ErrorKind,
	},
	path::{
		PathBuf,
	},
};

const DEFAULT_DATA_DIR: &str = "./computed-metrics";

pub fn resolve_path(filename: &str) -> Result<PathBuf, Box<dyn Error>> {
	let path: PathBuf = format!("{DEFAULT_DATA_DIR}/{filename}").into();
	if !path.exists() {
		Err(IoError::new(
			ErrorKind::NotFound,
			format!(
				"Could not find supplied file at: {DEFAULT_DATA_DIR}/{filename}"
			),
		)).into()
	}

	Ok(path)
}
