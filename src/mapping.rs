use burn::data::dataset::{
	Dataset,
	transform::Mapper,
};
use dataset_loader::{
	self,
	augmentation::{
		ImageStep,
		blur::Gaussian,
	},
	item::{
		LaserItem,
		controls::GeneralCameraControl,
	},
	sources::{
		StandardItem,
		StandardLaserDataset,
	},
};
use rand::{
	SeedableRng,
	rngs::StdRng,
};

use crate::{
	metric_extraction::{
		collect_metrics,
		compute_brightness_stats,
	},
	types::{
		CSVRowItem,
		CSVRows,
		GroupDef,
		ItemData,
		LoadedItem,
		LoadedItemMap,
	},
};

pub struct StandardItemMapper {
	blur: Gaussian,
}

impl StandardItemMapper {
	pub fn new(blur: Gaussian) -> Self {
		Self {
			blur,
		}
	}
}

impl Mapper<StandardItem, LoadedItem> for StandardItemMapper {
	fn map(
		&self,
		item: &StandardItem,
	) -> LoadedItem {
		let parents = item.get_image_path().get_parents();
		let position = parents.get(1).expect(
			"the image should have a grandparent directory for the position",
		);

		let intensity = *item.get_intensity();

		let image = item
			.load()
			.expect("The image failed to load apparently")
			.into_luma8();

		let mut rng = StdRng::seed_from_u64(0);
		let image = self.blur.apply(image, &mut rng);

		let group = GroupDef::new(
			position.to_string(),
			get_camera_control_value(item.white_balance()),
			get_camera_control_value(item.brightness()),
			get_camera_control_value(item.contrast()),
			get_camera_control_value(item.hue()),
			get_camera_control_value(item.saturation()),
			get_camera_control_value(item.sharpness()),
			get_camera_control_value(item.gamma()),
			get_camera_control_value(item.gain()),
			get_camera_control_value(item.exposure_absolute()),
		);

		let (peak, mean, std_dev) = compute_brightness_stats(&image);

		let image_data = ItemData::new(image, intensity, peak, mean, std_dev);

		LoadedItem {
			group,
			image_data,
		}
	}
}

fn get_camera_control_value(
	control: Option<GeneralCameraControl>
) -> Option<i64> {
	match control {
		Some(inner) => Some(*inner.value()),
		None => None,
	}
}

pub fn build_hashmap(
	dataset: StandardLaserDataset<StandardItemMapper, LoadedItem>
) -> LoadedItemMap {
	let hashmap: LoadedItemMap = dataset
		.iter()
		.map(|item: LoadedItem| (item.group, item.image_data))
		.fold(LoadedItemMap::new(), |mut map, (group, data)| {
			map.entry(group).or_default().push(data);
			map
		});

	hashmap
}

pub fn hashmap_to_rows(map: &LoadedItemMap) -> CSVRows {
	let mut rows = Vec::new();
	for (group, items) in map {
		for item in items {
			for setpoint in items {
				let (
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
				) = collect_metrics(&item, &setpoint);

				let row = CSVRowItem::new(
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
					item.intensity(),
					setpoint.intensity(),
					item.peak().into(),
					item.mean(),
					item.std_dev(),
					group.white_balance().map(|value| value as f32),
					group.brightness().map(|value| value as f32),
					group.contrast().map(|value| value as f32),
					group.hue().map(|value| value as f32),
					group.saturation().map(|value| value as f32),
					group.sharpness().map(|value| value as f32),
					group.gamma().map(|value| value as f32),
					group.gain().map(|value| value as f32),
					group.exposure_absolute().map(|value| value as f32),
				);
				rows.push(row)
			}
		}
	}
	rows
}
