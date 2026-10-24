use clap::{
	Parser,
	ValueEnum,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum Dataset {
	Synthetic1,
	Synthetic2,
	Analogous,
	#[value(name = "in-lab-single")]
	InLabSingle,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Sigma {
	#[value(name = "0.0", alias = "0")]
	S00,
	#[value(name = "0.8")]
	S08,
	#[value(name = "1.1")]
	S11,
	#[value(name = "1.4")]
	S14,
	#[value(name = "1.7")]
	S17,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TrainingMethod {
	LinearRegression,
	RidgeRegression,
	Lasso,
	ElasticNet,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Feature {
	PeakDiff,
	MeanDiff,
	StdDevDiff,
	LumaSSIM,
	Hellinger,
	Correlation,
	Intersection,
}

#[derive(Debug, Parser)]
pub struct TrainArgs {
	/// The name of the .csv file that contains the data used to train your
	/// model. The file must be inside the computed-metrics folder and must
	/// have been built with this tool.
	#[arg(long)]
	data: String,

	/// The type of regression to use. For more infomartion, see here:
	/// https://smartcorelib.org/user_guide/supervised.html#linear-models
	#[arg(long)]
	method: TrainingMethod,

	/// Features to be excluded from training this model.
	/// If both this flag and --include is omitted, all features will be
	/// included. This flag is ignored if --include is provided.
	#[arg(long, num_args = 0..)]
	exclude: Vec<Feature>,

	/// Features to be included in training this model.
	/// If both this flag and --include is omitted, all features will be
	/// included. Providing this flag will cause the --exclude flag to be
	/// ignored.
	#[arg(long, num_args = 0..)]
	include: Vec<Feature>,

	/// The exponent to be placed on the feature peak_diff.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "peak")]
	peak_diff_exp: i8,

	/// The exponent to be placed on the feature mean_diff.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "mean")]
	mean_diff_exp: i8,

	/// The exponent to be placed on the feature std_dev_diff.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "std-dev")]
	std_dev_diff_exp: i8,

	/// The exponent to be placed on the feature luma_ssim.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "ssim")]
	luma_ssim_exp: i8,

	/// The exponent to be placed on the feature hellinger.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "hellinger")]
	hellinger_exp: i8,

	/// The exponent to be placed on the feature correlation.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "correlation")]
	correlation_exp: i8,

	/// The exponent to be placed on the feature intersection.
	/// Defaults to 1.
	#[arg(long, default_value_t = 1, name = "intersection")]
	intersection_exp: i8,

	// TODO:
	// Add an exponent for each x field in CSVRowItem.
	// They should all be i8 to limit the exponent.
	/// Only data items with saturation values above (inclusive) this value
	/// will be used in training the model.
	#[arg(long, default_value_t = 0)]
	saturation_min: u16,

	/// Only data items with saturation values below (inclusive) this value
	/// will be used in training the model.
	#[arg(long, default_value_t = u16::MAX)]
	saturation_max: u16,

	/// Only data items with exposure values above (inclusive) this value will
	/// be used in training the model.
	#[arg(long, default_value_t = 0)]
	exposure_min: u16,

	/// Only data items with exposure values below (inclusive) this value will
	/// be used in training the model.
	#[arg(long, default_value_t = u16::MAX)]
	exposure_max: u16,

	/// Only data items with gamma values above (inclusive) this value will
	/// be used in training the model.
	#[arg(long, default_value_t = 0)]
	gamma_min: u16,

	/// Only data items with exposure values below (inclusive) this value will
	/// be used in training the model.
	#[arg(long, default_value_t = u16::MAX)]
	gamma_max: u16,
	// Each camera control should be represented here
}

impl TrainArgs {
	pub fn data(&self) -> &str {
		&self.data
	}

	pub fn method(&self) -> &TrainingMethod {
		&self.method
	}

	pub fn excluded_features(&self) -> &[Feature] {
		&self.exclude
	}

	pub fn included_features(&self) -> &[Feature] {
		&self.include
	}

	pub fn peak_diff_exp(&self) -> i8 {
		self.peak_diff_exp
	}

	pub fn mean_diff_exp(&self) -> i8 {
		self.mean_diff_exp
	}

	pub fn std_dev_diff_exp(&self) -> i8 {
		self.std_dev_diff_exp
	}

	pub fn luma_ssim_exp(&self) -> i8 {
		self.luma_ssim_exp
	}

	pub fn hellinger_exp(&self) -> i8 {
		self.hellinger_exp
	}

	pub fn corellation_exp(&self) -> i8 {
		self.correlation_exp
	}

	pub fn intersection_exp(&self) -> i8 {
		self.intersection_exp
	}

	// TODO: Add the rest all the exponents for fields as seen in CSVRowItem
}

pub fn get_train_args() -> TrainArgs {
	TrainArgs::parse()
}
