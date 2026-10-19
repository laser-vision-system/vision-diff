use burn::data::dataset::transform::Mapper;
use dataset_loader::{
	augmentation::{
		ImageStep,
		blur::Gaussian,
	},
	item::LaserItem,
	sources::{
		analogous_laser::AnalogousLaserItem,
		in_lab_single_laser::InLabSingleLaserItem,
	},
};
use image::{
	GrayImage,
};
use rand::{
	SeedableRng,
	rngs::StdRng,
};

use crate::metric_extraction::compute_brightness_stats;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupDef {
	position: String,
	saturation: u8,
	exposure: u16,
	gamma: u16,
}

impl GroupDef {
    pub fn position(&self) -> &str {
        &self.position
    }
    pub fn saturation(&self) -> u8 {
        self.saturation
    }
    pub fn exposure(&self) -> u16 {
        self.exposure
    }
    pub fn gamma(&self) -> u16 {
        self.gamma
    }
}

#[derive(Clone, Debug)]
pub struct ItemData {
	image: GrayImage,
	intensity: f32,
	peak: u8,
	mean: f32,
	std_dev: f32,
}

impl ItemData {
    pub fn image(&self) -> &GrayImage {
        &self.image
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
    pub fn peak(&self) -> u8 {
        self.peak
    }
    pub fn mean(&self) -> f32 {
        self.mean
    }
    pub fn std_dev(&self) -> f32 {
        self.std_dev
    }
}

#[derive(Clone, Debug)]
pub struct LoadedItem {
	pub group: GroupDef,
	pub image_data: ItemData,
}

pub struct AnalogousItemMapper {
	blur: Gaussian,
}

impl AnalogousItemMapper {
	pub fn new(blur: Gaussian) -> Self {
		Self {
			blur,
		}
	}
}

impl Mapper<AnalogousLaserItem, LoadedItem> for AnalogousItemMapper {
	fn map(
		&self,
		item: &AnalogousLaserItem,
	) -> LoadedItem {
		let parents = item.get_image_path().get_parents();
		let position = parents.get(1).expect(
			"the image should have a grandparent directory for the position",
		);

		let intensity = *item.get_intensity();
		let saturation = item.get_saturation();
		let exposure = item.get_exposure_absolute();
		let gamma = item.get_gamma();

		let image = item
			.load()
			.expect("The image failed to load apparently")
			.into_luma8();

		let mut rng = StdRng::seed_from_u64(0);
		let image = self.blur.apply(image, &mut rng);

		let group = GroupDef {
			position: position.to_string(),
			saturation,
			exposure,
			gamma,
		};

		let (peak, mean, std_dev) = compute_brightness_stats(&image);

		let image_data = ItemData {
			image,
			intensity,
			peak,
			mean,
			std_dev,
		};

		LoadedItem {
			group,
			image_data,
		}
	}
}

pub struct InLabSingleItemMapper {
	blur: Gaussian,
}

impl InLabSingleItemMapper {
	pub fn new(blur: Gaussian) -> Self {
		Self {
			blur,
		}
	}
}

impl Mapper<InLabSingleLaserItem, LoadedItem> for InLabSingleItemMapper {
	fn map(
		&self,
		item: &InLabSingleLaserItem,
	) -> LoadedItem {
		let parents = item.get_image_path().get_parents();
		let position = parents.get(1).expect(
			"the image should have a grandparent directory for the position",
		);

		let intensity = *item.get_intensity();
		let saturation = item.get_saturation();
		let exposure = item.get_exposure_absolute();
		let gamma = item.get_gamma();

		let image = item
			.load()
			.expect("The image failed to load apparently")
			.into_luma8();

		let mut rng = StdRng::seed_from_u64(0);
		let image = self.blur.apply(image, &mut rng);

		let group = GroupDef {
			position: position.to_string(),
			saturation,
			exposure,
			gamma,
		};

		let (peak, mean, std_dev) = compute_brightness_stats(&image);

		let image_data = ItemData {
			image,
			intensity,
			peak,
			mean,
			std_dev,
		};

		LoadedItem {
			group,
			image_data,
		}
	}
}
