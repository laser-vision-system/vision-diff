use std::collections::HashMap;

use clap::ValueEnum;
use image::GrayImage;
use serde::{
	Deserialize,
	Serialize,
};
use strum::EnumIter;

use crate::config::{
	FeatureConfig,
	Model,
};

/// Ths single source of truth for any .csv file created by the csv builder part
/// of this application and the expected format of any .csv file used to build a
/// model using this application.
///
/// Is tied to the Column enum.
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct CSVRowItem {
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
	white_balance: Option<f32>,
	brightness: Option<f32>,
	contrast: Option<f32>,
	hue: Option<f32>,
	saturation: Option<f32>,
	sharpness: Option<f32>,
	gamma: Option<f32>,
	gain: Option<f32>,
	exposure_absolute: Option<f32>,
}

impl CSVRowItem {
	pub fn new(
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
		white_balance: Option<f32>,
		brightness: Option<f32>,
		contrast: Option<f32>,
		hue: Option<f32>,
		saturation: Option<f32>,
		sharpness: Option<f32>,
		gamma: Option<f32>,
		gain: Option<f32>,
		exposure_absolute: Option<f32>,
	) -> Self {
		CSVRowItem {
			peak_diff,
			mean_diff,
			std_dev_diff,
			luma_ssim,
			hellinger,
			correlation,
			intersection,
			intensity,
			setpoint_intensity,
			peak,
			mean,
			std_dev,
			white_balance,
			brightness,
			contrast,
			hue,
			saturation,
			sharpness,
			gamma,
			gain,
			exposure_absolute,
		}
	}

	pub fn get(
		&self,
		field: &Column,
	) -> Option<f32> {
		match field {
			Column::PeakDiff => Some(self.peak_diff),
			Column::MeanDiff => Some(self.mean_diff),
			Column::StdDevDiff => Some(self.std_dev_diff),
			Column::LumaSSIM => Some(self.luma_ssim),
			Column::Hellinger => Some(self.hellinger),
			Column::Correlation => Some(self.correlation),
			Column::Intersection => Some(self.intersection),
			Column::Intensity => Some(self.intensity),
			Column::SetpointIntensity => Some(self.setpoint_intensity),
			Column::Peak => Some(self.peak),
			Column::Mean => Some(self.mean),
			Column::StdDev => Some(self.std_dev),
			Column::WhiteBalance => self.white_balance,
			Column::Brightness => self.brightness,
			Column::Contrast => self.contrast,
			Column::Hue => self.hue,
			Column::Saturation => self.saturation,
			Column::Sharpness => self.sharpness,
			Column::Gamma => self.gamma,
			Column::Gain => self.gain,
			Column::ExposureAbsolute => self.exposure_absolute,
		}
	}
}

#[repr(usize)]
#[derive(
	Debug,
	Clone,
	Copy,
	ValueEnum,
	EnumIter,
	Eq,
	PartialEq,
	Hash,
	Serialize,
	Deserialize,
)]
pub enum Column {
	PeakDiff = 0,
	MeanDiff = 1,
	StdDevDiff = 2,
	LumaSSIM = 3,
	Hellinger = 4,
	Correlation = 5,
	Intersection = 6,
	Intensity = 7,
	SetpointIntensity = 8,
	Peak = 9,
	Mean = 10,
	StdDev = 11,
	WhiteBalance = 12,
	Brightness = 13,
	Contrast = 14,
	Hue = 15,
	Saturation = 16,
	Sharpness = 17,
	Gamma = 18,
	Gain = 19,
	ExposureAbsolute = 20,
}

impl Column {
	pub fn to_string(&self) -> String {
		match self {
			Column::PeakDiff => "peak_diff".to_string(),
			Column::MeanDiff => "mean_diff".to_string(),
			Column::StdDevDiff => "std_dev_diff".to_string(),
			Column::LumaSSIM => "luma_ssim".to_string(),
			Column::Hellinger => "hellinger".to_string(),
			Column::Correlation => "correlation".to_string(),
			Column::Intersection => "intersection".to_string(),
			Column::Intensity => "intensity".to_string(),
			Column::SetpointIntensity => "setpoint_intensity".to_string(),
			Column::Peak => "peak".to_string(),
			Column::Mean => "mean".to_string(),
			Column::StdDev => "std_dev".to_string(),
			Column::WhiteBalance => "white_balance".to_string(),
			Column::Brightness => "brightness".to_string(),
			Column::Contrast => "contrast".to_string(),
			Column::Hue => "hue".to_string(),
			Column::Saturation => "saturation".to_string(),
			Column::Sharpness => "sharpness".to_string(),
			Column::Gamma => "gamma".to_string(),
			Column::Gain => "gain".to_string(),
			Column::ExposureAbsolute => "exposure_absolute".to_string(),
		}
	}
}

impl TryFrom<&str> for Column {
	type Error = String;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"peak_diff" => Ok(Column::PeakDiff),
			"mean_diff" => Ok(Column::MeanDiff),
			"std_dev_diff" => Ok(Column::StdDevDiff),
			"luma_ssim" => Ok(Column::LumaSSIM),
			"hellinger" => Ok(Column::Hellinger),
			"correlation" => Ok(Column::Correlation),
			"intersection" => Ok(Column::Intersection),
			"intensity" => Ok(Column::Intensity),
			"setpoint_intensity" => Ok(Column::SetpointIntensity),
			"peak" => Ok(Column::Peak),
			"mean" => Ok(Column::Mean),
			"std_dev" => Ok(Column::StdDev),
			"white_balance" => Ok(Column::WhiteBalance),
			"brightness" => Ok(Column::Brightness),
			"contrast" => Ok(Column::Contrast),
			"hue" => Ok(Column::Hue),
			"saturation" => Ok(Column::Saturation),
			"sharpness" => Ok(Column::Sharpness),
			"gamma" => Ok(Column::Gamma),
			"gain" => Ok(Column::Gain),
			"exposure_absolute" => Ok(Column::ExposureAbsolute),
			_ => Err(format!("Unknown column variant: {value}")),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct ModelOutput {
	model: Model,
	features: Vec<FeatureOutput>,
	intercept: f32,
	accuracy: Accuracy,
}

impl ModelOutput {
	pub fn new(
		model: Model,
		features: Vec<FeatureOutput>,
		intercept: f32,
		accuracy: Accuracy,
	) -> Self {
		ModelOutput {
			model,
			features,
			intercept,
			accuracy,
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct FeatureOutput {
	feature: Column,
	coefficient: f32,
	min: Option<f32>,
	max: Option<f32>,
	exponent: f32,
}

impl FeatureOutput {
	pub fn new(
		feature: Column,
		coefficient: f32,
		min: Option<f32>,
		max: Option<f32>,
		exponent: f32,
	) -> Self {
		FeatureOutput {
			feature,
			coefficient,
			min,
			max,
			exponent,
		}
	}
}

impl From<(Column, f32, FeatureConfig)> for FeatureOutput {
	fn from(value: (Column, f32, FeatureConfig)) -> Self {
		FeatureOutput {
			feature: value.0,
			coefficient: value.1,
			min: value.2.min(),
			max: value.2.max(),
			exponent: value.2.exponent(),
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct Accuracy {
	mean_absolute_error: f64,
	mean_squared_error: f64,
	r_squared: f64,
}

impl Accuracy {
	pub fn new(
		mean_absolute_error: f64,
		mean_squared_error: f64,
		r_squared: f64,
	) -> Self {
		Accuracy {
			mean_absolute_error,
			mean_squared_error,
			r_squared,
		}
	}
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupDef {
	position: String,
	white_balance: Option<i64>,
	brightness: Option<i64>,
	contrast: Option<i64>,
	hue: Option<i64>,
	saturation: Option<i64>,
	sharpness: Option<i64>,
	gamma: Option<i64>,
	gain: Option<i64>,
	exposure_absolute: Option<i64>,
}

impl GroupDef {
	pub fn new(
		position: String,
		white_balance: Option<i64>,
		brightness: Option<i64>,
		contrast: Option<i64>,
		hue: Option<i64>,
		saturation: Option<i64>,
		sharpness: Option<i64>,
		gamma: Option<i64>,
		gain: Option<i64>,
		exposure_absolute: Option<i64>,
	) -> Self {
		GroupDef {
			position,
			white_balance,
			brightness,
			contrast,
			hue,
			saturation,
			sharpness,
			gamma,
			gain,
			exposure_absolute,
		}
	}

	pub fn position(&self) -> &str {
		&self.position
	}

	pub fn white_balance(&self) -> Option<i64> {
		self.white_balance
	}

	pub fn brightness(&self) -> Option<i64> {
		self.brightness
	}

	pub fn contrast(&self) -> Option<i64> {
		self.contrast
	}

	pub fn hue(&self) -> Option<i64> {
		self.hue
	}

	pub fn saturation(&self) -> Option<i64> {
		self.saturation
	}

	pub fn sharpness(&self) -> Option<i64> {
		self.sharpness
	}

	pub fn gamma(&self) -> Option<i64> {
		self.gamma
	}

	pub fn gain(&self) -> Option<i64> {
		self.gain
	}

	pub fn exposure_absolute(&self) -> Option<i64> {
		self.exposure_absolute
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
	pub fn new(
		image: GrayImage,
		intensity: f32,
		peak: u8,
		mean: f32,
		std_dev: f32,
	) -> Self {
		ItemData {
			image,
			intensity,
			peak,
			mean,
			std_dev,
		}
	}

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

pub type LoadedItemMap = HashMap<GroupDef, Vec<ItemData>>;

pub type CSVRows = Vec<CSVRowItem>;
