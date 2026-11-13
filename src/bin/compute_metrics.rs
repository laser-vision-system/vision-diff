use std::{
	error::Error,
	path::PathBuf,
	str::FromStr,
};

use csv::Writer;
use dataset_loader::augmentation::blur::{
	BlurSigma,
	Gaussian,
};
use vision_diff::{
	config::{
		DEFAULT_COMPUTE_OUTPUT_DIR,
		STATIC_COMPUTE_CONFIG,
		read_compute_metrics_config,
	},
	dataset::standard_dataset,
	mapping::{
		build_hashmap,
		hashmap_to_rows,
	},
};

fn main() -> Result<(), Box<dyn Error>> {
	let config = read_compute_metrics_config()?;

	for blur in config.blur_sigmas() {
		let gaussian = Gaussian::new(BlurSigma::try_new(*blur)?);

		let output_path = PathBuf::from_str(DEFAULT_COMPUTE_OUTPUT_DIR)?
			.join(format!("{}_sigma-{}.csv", config.output_name(), blur));

		if output_path.exists() {
			return Err(format!(
				"Could not create dataset at: {:?}. File already exists!",
				output_path
			)
			.into());
		}

		let mut writer = Writer::from_path(output_path)?;

		let rows = hashmap_to_rows(&build_hashmap(standard_dataset(
			gaussian,
			STATIC_COMPUTE_CONFIG.get_or_init(|| config.clone()),
		)?));

		for row in rows {
			writer.serialize(row)?;
		}
	}

	Ok(())
}
