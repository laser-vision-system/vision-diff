use clap::ValueEnum;

pub mod command;
pub mod dataset;
pub mod fit;
pub mod mapping;
pub mod metric_extraction;
pub mod path;
pub mod transform;

/// Ths single source of truth for any .csv file created by the csv builder part
/// of this application and the expected format of any .csv file used to build a
/// model using this application.
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
}

#[repr(usize)]
#[derive(Debug, Clone, ValueEnum)]
pub enum Column {
	Saturation = 0,
	Exposure = 1,
	Gamma = 2,
	Peak = 3,
	Mean = 4,
	StdDev = 5,
	Intensity = 6,
	SetpointIntensity = 7,
	PeakDiff = 8,
	MeanDiff = 9,
	StdDevDiff = 10,
	LumaSSIM = 11,
	Hellinger = 12,
	Correlation = 13,
	Intersection = 14,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TrainingMethod {
	LinearRegression,
	RidgeRegression,
	Lasso,
	ElasticNet,
}
