use image::GrayImage;
use image_compare::{
	Algorithm,
	Metric,
	gray_similarity_histogram,
	gray_similarity_structure,
};

use crate::types::ItemData;

pub fn compute_brightness_stats(img: &GrayImage) -> (u8, f32, f32) {
	let pixels: Vec<u8> = img.pixels().map(|p| p.0[0]).collect();
	let n = pixels.len() as f32;

	let sum: f32 = pixels.iter().map(|&p| p as f32).sum();
	let mean = sum / n;

	let variance = pixels
		.iter()
		.map(|&p| {
			let d = p as f32 - mean;
			d * d
		})
		.sum::<f32>()
		/ n;

	let std_dev = variance.sqrt();

	let peak = pixels.iter().max().copied().unwrap_or(0);

	(peak, mean, std_dev)
}

pub fn collect_metrics(
	item: &ItemData,
	setpoint: &ItemData,
) -> (f32, f32, f32, f32, f32, f32, f32) {
	let (item_peak, item_mean, item_std_dev) =
		(item.peak(), item.mean(), item.std_dev());
	let (setpoint_peak, setpoint_mean, setpoint_std_dev) =
		(setpoint.peak(), setpoint.mean(), setpoint.std_dev());
	let luma_ssim = gray_similarity_structure(
		&Algorithm::MSSIMSimple,
		item.image(),
		setpoint.image(),
	)
	.unwrap()
	.score;
	let hellinger = gray_similarity_histogram(
		Metric::Hellinger,
		item.image(),
		setpoint.image(),
	)
	.unwrap();
	let correlation = gray_similarity_histogram(
		Metric::Correlation,
		item.image(),
		setpoint.image(),
	)
	.unwrap();
	let intersection = gray_similarity_histogram(
		Metric::Intersection,
		item.image(),
		setpoint.image(),
	)
	.unwrap();

	#[cfg(debug_assertions)]
	println!("Metrics collected!");

	return (
		setpoint_peak as f32 - item_peak as f32,
		setpoint_mean - item_mean,
		setpoint_std_dev - item_std_dev,
		luma_ssim as f32,
		hellinger as f32,
		correlation as f32,
		intersection as f32,
	);
}
