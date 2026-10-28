use std::{
	error::Error,
	io::{
		Error as IoError,
		ErrorKind,
	},
	path::{
		Path,
		PathBuf,
	},
};

pub fn file_exists(
	dir: &str,
	filename: &str,
) -> Result<PathBuf, Box<dyn Error>> {
	let path: PathBuf = Path::new(dir).join(filename).into();
	if !path.exists() {
		return Err(IoError::new(
			ErrorKind::NotFound,
			format!("Could not find supplied file at: {dir}/{filename}"),
		)
		.into());
	}

	Ok(path)
}
