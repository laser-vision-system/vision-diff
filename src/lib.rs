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
/// 
/// Is tied to the Column eum.
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
	pub fn get(&self, field: Column) -> f32 {
		match field {
			PeakDiff => self.peak_diff,
			MeanDiff => self.mean_diff,
			StdDevDiff => self.std_dev_diff,
			LumaSSIM => self.luma_ssim,
			Hellinger => self.hellinger,
			Correlation => self.correlation,
			Intersection => self.intersection,
			Intensity => self.intensity,
			SetpointIntensity => self.setpoint_intensity,
			Peak => self.peak,
			Mean => self.mean,
			StdDev => self.std_dev,
			WhiteBalance => self.white_balance,
			Brightness => self.brightness,
			Contrast => self.contrast,
			Hue => self.hue,
			Saturation => self.saturation,
			Sharpness => self.sharpness,
			Gamma => self.gamma,
			Gain => self.gain,
			ExposureAbsolute => self.exposure_absolute,
		}
	}
}

#[repr(usize)]
#[derive(Debug, Clone, ValueEnum)]
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

#[derive(Debug, Clone, ValueEnum)]
pub enum TrainingMethod {
	LinearRegression,
	RidgeRegression,
	Lasso,
	ElasticNet,
}
