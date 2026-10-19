use dataset_loader::{
	self,
	augmentation::blur::Gaussian,
	dataset::LaserDataset,
	item::LaserItem,
	sources::{
		analogous_laser::{
			self,
			AnalogousLaserItem,
		},
		in_lab_single_laser::{
			self,
			InLabSingleLaserItem,
		},
	},
};

use crate::mapping::{
	AnalogousItemMapper,
	InLabSingleItemMapper,
	LoadedItem,
};

pub fn analogous_dataset(
	blur: Gaussian
) -> LaserDataset<AnalogousItemMapper, AnalogousLaserItem, LoadedItem> {
	let mapper = AnalogousItemMapper::new(blur);
	let root = "datasets/analogous_laser";
	let dataset = match analogous_laser::get_dataset(
		mapper,
		root,
		&[Box::new(|item| {
			let parents = item.get_image_path().get_parents();
			let grandparent = parents.get(1).expect(
				"the image should have a grandparent directory for the position",
			);
			grandparent != "rearwards_close_outer_0-impure"
				&& grandparent != "rearwards_far_outer_270"
				&& grandparent != "not_applicable"
		})],
	) {
		Ok(value) => value,
		Err(error) => {
			eprintln!("Error: {error}");
			panic!();
		},
	};
	dataset
}

pub fn in_lab_single_dataset(
	blur: Gaussian
) -> LaserDataset<InLabSingleItemMapper, InLabSingleLaserItem, LoadedItem> {
	let mapper = InLabSingleItemMapper::new(blur);
	let root = "datasets/in_lab_single_laser";
	let dataset = match in_lab_single_laser::get_dataset(
		mapper,
		root,
		&[Box::new(|item| {
			let parents = item.get_image_path().get_parents();
			let grandparent = parents.get(1).expect(
				"the image should have a grandparent directory for the position",
			);
			grandparent != "rearwards_close_outer_0-impure"
				&& grandparent != "rearwards_far_outer_270"
				&& grandparent != "not_applicable"
		})],
	) {
		Ok(value) => value,
		Err(error) => {
			eprintln!("Error: {error}");
			panic!();
		},
	};
	dataset
}
