use std::{
	error::Error,
	path::PathBuf,
	str::FromStr,
};

use csv::{
	Reader,
	Writer,
};
use serde::{
	Deserialize,
	Serialize,
};

fn main() -> Result<(), Box<dyn Error>> {
	let folder = PathBuf::from_str("./computed-metrics")?;
	let old_file = folder.join("legacy_in-lab-single_sigma-1.7.csv");
	let new_file = folder.join("in-lab-single_sigma-1.7.csv");

	let mut reader = Reader::from_path(old_file)?;
	let mut writer = Writer::from_path(new_file)?;

	for result in reader.deserialize() {
		let row: OldRowItem = result?;

		let output: NewRowItem = row.into();

        writer.serialize(output)?;
	}

	Ok(())
}

#[derive(Serialize, Deserialize)]
struct OldRowItem {
	position: String,
	saturation: f32,
	exposure: f32,
	gamma: f32,
	intensity: f32,
	setpoint_intensity: f32,
	peak_diff: f32,
	mean_diff: f32,
	std_dev_diff: f32,
	luma_ssim: f32,
	hellinger: f32,
	correlation: f32,
	intersection: f32,
}

#[derive(Serialize, Deserialize)]
struct NewRowItem {
    position: String,
	peak_diff: f32,
	mean_diff: f32,
	std_dev_diff: f32,
	luma_ssim: f32,
	hellinger: f32,
	correlation: f32,
	intersection: f32,
	intensity: f32,
	setpoint_intensity: f32,
	peak: f32,
	mean: f32,
	std_dev: f32,
	white_balance: f32,
	brightness: f32,
	contrast: f32,
	hue: f32,
	saturation: f32,
	sharpness: f32,
	gamma: f32,
	gain: f32,
	exposure_absolute: f32,
}

impl From<OldRowItem> for NewRowItem {
	fn from(value: OldRowItem) -> Self {
		NewRowItem {
            position: value.position,
			peak_diff: value.peak_diff,
			mean_diff: value.mean_diff,
			std_dev_diff: value.std_dev_diff,
			luma_ssim: value.luma_ssim,
			hellinger: value.hellinger,
			correlation: value.correlation,
			intersection: value.intersection,
			intensity: value.intensity,
			setpoint_intensity: value.setpoint_intensity,
			peak: 0.0,
			mean: 0.0,
			std_dev: 0.0,
			white_balance: 0.0,
			brightness: 0.0,
			contrast: 0.0,
			hue: 0.0,
			saturation: 0.0,
			sharpness: 0.0,
			gamma: 0.0,
			gain: 0.0,
			exposure_absolute: 0.0,
		}
	}
}
