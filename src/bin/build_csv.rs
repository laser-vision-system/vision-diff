use std::{
	collections::HashMap,
	error::Error,
	io::Write,
};

use burn::{
	data::dataset::{
		Dataset,
		transform::Mapper,
	},
	serde::Serialize,
};
use csv::Writer;
use dataset_loader::{
	augmentation::blur::{
		BlurSigma,
		Gaussian,
	},
	dataset::LaserDataset,
	item::LaserItem,
};
use vision_diff::{
	dataset::{
		analogous_dataset,
		in_lab_single_dataset,
	},
	mapping::{
		GroupDef,
		ItemData,
		LoadedItem,
	},
	metric_extraction::collect_metrics,
};

type LoadedItemMap = HashMap<GroupDef, Vec<ItemData>>;

type CSVRows = Vec<CSVItem>;

#[derive(Serialize)]
struct CSVItem {
	position: String,
	saturation: u8,
	exposure: u16,
	gamma: u16,
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

fn loaded_item_map<M, MI>(
	dataset: LaserDataset<M, MI, LoadedItem>
) -> LoadedItemMap
where
	M: Mapper<MI, LoadedItem> + Send + Sync,
	MI: Send + Sync + Clone + LaserItem,
{
	let hashmap: LoadedItemMap = dataset
		.iter()
		.map(|item: LoadedItem| (item.group, item.image_data))
		.fold(LoadedItemMap::new(), |mut map, (group, data)| {
			map.entry(group).or_default().push(data);
			map
		});

	hashmap
}

fn convert_map(map: &LoadedItemMap) -> CSVRows {
	let mut rows = Vec::new();
	for (group, items) in map {
		for item in items {
			for setpoint in items {
				let position = group.position().to_string();
				let saturation = group.saturation();
				let exposure = group.exposure();
				let gamma = group.gamma();
				let intensity = item.intensity();
				let setpoint_intensity = setpoint.intensity();
				let (
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
				) = collect_metrics(&item, &setpoint);
				rows.push(CSVItem {
					position,
					saturation,
					exposure,
					gamma,
					intensity,
					setpoint_intensity,
					peak_diff,
					mean_diff,
					std_dev_diff,
					luma_ssim,
					hellinger,
					correlation,
					intersection,
				})
			}
		}
	}
	rows
}

fn write_csv<W>(
	mut writer: Writer<W>,
	rows: CSVRows,
) -> Result<(), Box<dyn Error>>
where
	W: Write,
{
	for row in rows {
		writer.serialize(row)?;
	}
	Ok(())
}

fn main() {
	let blurs: Vec<f32> = vec![0.0, 0.8, 1.1, 1.4, 1.7];

	for blur in blurs {
		let gaussian = Gaussian::new(
			BlurSigma::try_new(blur)
				.expect("Hardcoded sigma values should work here."),
		);
		let writer = Writer::from_path(format!(
			"csv/in-lab-single-laser_sigma-{}.csv",
			blur
		))
		.expect("The CSV writer should be able to construct.");
		write_csv(
			writer,
			convert_map(&loaded_item_map(in_lab_single_dataset(gaussian))),
		)
		.unwrap_or_else(|err| {
			eprintln!(
				"Something probably went wrong while writing a row: {err}"
			)
		});
	}

	let blurs: Vec<f32> = vec![0.0, 0.8, 1.1, 1.4, 1.7];

	for blur in blurs {
		let gaussian = Gaussian::new(
			BlurSigma::try_new(blur)
				.expect("Hardcoded sigma values should work here."),
		);
		let writer =
			Writer::from_path(format!("csv/analogous_sigma-{}.csv", blur))
				.expect("The CSV writer should be able to construct.");
		write_csv(
			writer,
			convert_map(&loaded_item_map(analogous_dataset(gaussian))),
		)
		.unwrap_or_else(|err| {
			eprintln!(
				"Something probably went wrong while writing a row: {err}"
			)
		});
	}
}
