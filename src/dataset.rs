use std::error::Error;

use dataset_loader::{
	self,
	augmentation::blur::Gaussian,
	item::LaserItem,
	sources::{
		StandardItem,
		StandardLaserDataset,
		build_standard_dataset,
	},
};

use crate::{
	config::{
		CameraControl,
		ComputeMetricsConfig,
		Dataset,
	},
	mapping::StandardItemMapper,
	transform::within_bounds,
	types::LoadedItem,
};

fn filter(
	item: &StandardItem,
	config: ComputeMetricsConfig,
) -> bool {
	let mut return_bool = true;
	let parent = item.get_image_path().get_parents();
	let grandparent = &parent
		.get(1)
		.expect(
			"There should always be a grandparent directory for images in the dataset.",
		)
		.as_ref()
		.to_string();

	return_bool =
		return_bool && config.exclude_position().contains(grandparent);

	for control in config.controls() {
		if let Some(value) = match control.control() {
			CameraControl::WhiteBalance => item.white_balance(),
			CameraControl::Brightness => item.brightness(),
			CameraControl::Contrast => item.contrast(),
			CameraControl::Hue => item.hue(),
			CameraControl::Saturation => item.saturation(),
			CameraControl::Sharpness => item.sharpness(),
			CameraControl::Gamma => item.gamma(),
			CameraControl::Gain => item.gain(),
			CameraControl::ExposureAbsolute => item.exposure_absolute(),
		} {
			return_bool = return_bool
				&& within_bounds(
					Some(*value.value() as f32),
					control.min(),
					control.max(),
				);
		}
	}
	return_bool
}

pub fn standard_dataset(
	blur: Gaussian,
	config: &'static ComputeMetricsConfig,
) -> Result<StandardLaserDataset<StandardItemMapper, LoadedItem>, Box<dyn Error>>
{
	let mapper = StandardItemMapper::new(blur);
	let root = match config.dataset() {
		Dataset::Synthetic1 => "./datasets/synthetic_1",
		Dataset::Synthetic2 => "./datasets/synthetic_2",
		Dataset::Analogous => "./datasets/analogous_laser",
		Dataset::InLabSingle => "./datasets/in_lab_single_laser",
	};
	Ok(build_standard_dataset(
		mapper,
		root,
		&[Box::new(|item| filter(item, config.clone()))],
	)?)
}
