use std::collections::HashMap;

use burn::data::dataset::transform::Mapper;
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
};

pub fn analogous_dataset(
	blur: Gaussian
) -> LaserDataset<AnalogousItemMapper, AnalogousLaserItem, LoadedItem> {
	#[cfg(debug_assertions)]
	println!("Creating an analogous dataset...");

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
			eprintln!("Error building dataset: {error}");
			panic!();
		},
	};
	dataset
}

pub fn in_lab_single_dataset(
	blur: Gaussian
) -> LaserDataset<InLabSingleItemMapper, InLabSingleLaserItem, LoadedItem> {
	#[cfg(debug_assertions)]
	println!("Creating an in-lab-single-laser dataset...");

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
				&& grandparent
					!= "forwards-incorrect_mount-0-impure-backplate-light_strips"
				&& grandparent
					!= "forwards-incorrect_mount-0-impure-no_backplate-light_strips"
				&& grandparent
					!= "forwards-top-0-impure-no_backplate-shutters_open"
				&& grandparent
					!= "forwards-top-0-impure-no_backplate-shutters_open-light_strips"
				&& grandparent
					!= "rearwards-incorrect_mount-0-impure-backplate-light_strips"
				&& grandparent
					!= "rearwards-incorrect_mount-0-impure-no_backplate-light_strips"
		})],
	) {
		Ok(value) => value,
		Err(error) => {
			eprintln!("Error building dataset: {error}");
			panic!();
		},
	};
	dataset
}

pub struct Dataset<M, MI, MO, G, D>
where
	M: Mapper<MI, MO> + Send + Sync,
	MI: Send + Sync + Clone + LaserItem,
	MO: Send + Sync + Clone + LoadedItem<G, D>,
{
	dataset: LaserDataset<M, MI, MO>,
}

impl<M, MI, MO, G, D> Dataset<M, MI, MO, G, D>
where
	M: Mapper<MI, MO> + Send + Sync,
	MI: Send + Sync + Clone + LaserItem,
	MO: Send + Sync + Clone + LoadedItem<G, D>,
{
	pub fn build_hashmap() -> HashMap<G, Vec<D>> {
	}
}
