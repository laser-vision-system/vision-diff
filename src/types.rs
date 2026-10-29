use clap::ValueEnum;
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
#[derive(PartialEq, Debug, Deserialize)]
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

impl CSVRowItem {
	pub fn get(
		&self,
		field: &Column,
	) -> f32 {
		match field {
			Column::PeakDiff => self.peak_diff,
			Column::MeanDiff => self.mean_diff,
			Column::StdDevDiff => self.std_dev_diff,
			Column::LumaSSIM => self.luma_ssim,
			Column::Hellinger => self.hellinger,
			Column::Correlation => self.correlation,
			Column::Intersection => self.intersection,
			Column::Intensity => self.intensity,
			Column::SetpointIntensity => self.setpoint_intensity,
			Column::Peak => self.peak,
			Column::Mean => self.mean,
			Column::StdDev => self.std_dev,
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
pub struct Output {
	model: Model,
	features: Vec<FeatureOutput>,
	intercept: f32,
	accuracy: Accuracy,
}

impl Output {
	pub fn new(
		model: Model,
		features: Vec<FeatureOutput>,
		intercept: f32,
		accuracy: Accuracy,
	) -> Self {
		Output {
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
