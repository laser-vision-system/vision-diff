use std::{
	error::Error,
	path::{
		Path,
		PathBuf,
	},
};

use clap::Parser;
use serde::{
	Deserialize,
	Serialize,
};
use smartcore::linear::{
	linear_regression::LinearRegressionSolverName,
	ridge_regression::RidgeRegressionSolverName,
};

use crate::types::Column;

const DEFAULT_DATA_DIR: &str = "./computed-metrics";
const DEFAULT_MODEL_DIR: &str = "./models";
const DEFAULT_TRAIN_CONFIG_DIR: &str = "./config/train";
const DEFAULT_COMPUTE_CONFIG_DIR: &str = "./config/compute";

#[derive(Debug, Parser)]
struct CliArgs {
	#[arg(long)]
	config: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
	input_data: String,
	output_name: String,
	test_size: f32,
	shuffle: bool,
	splitting_seed: Option<u64>,
}

impl GeneralConfig {
	pub fn output_name(&self) -> &str {
		&self.output_name
	}

	pub fn shuffle(&self) -> bool {
		self.shuffle
	}

	pub fn test_size(&self) -> f32 {
		self.test_size
	}

	pub fn splitting_seed(&self) -> Option<u64> {
		self.splitting_seed
	}
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct FeatureConfig {
	feature: Column,
	include: bool,
	min: Option<f32>,
	max: Option<f32>,

	#[serde(default = "default_exp")]
	exponent: f32,
}

impl FeatureConfig {
	pub fn column(&self) -> &Column {
		&self.feature
	}

	pub fn include(&self) -> bool {
		self.include
	}

	pub fn min(&self) -> Option<f32> {
		self.min
	}

	pub fn max(&self) -> Option<f32> {
		self.max
	}

	pub fn exponent(&self) -> f32 {
		self.exponent
	}
}

fn default_exp() -> f32 {
	1.0
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Model {
	LinearRegression {
		solver: LinearRegressionSolver,
	},
	RidgeRegression {
		solver: RidgeRegressionSolver,
		alpha: f32,
		normalize: bool,
	},
	Lasso {
		alpha: f32,
		normalize: bool,
		tol: f32,
		max_iter: u32,
		fit_intercept: bool,
	},
	ElasticNet {
		alpha: f32,
		l1_ratio: f32,
		normalize: bool,
		tol: f32,
		max_iter: u32,
	},
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum LinearRegressionSolver {
	QR,
	SVD,
}

impl From<&LinearRegressionSolver> for LinearRegressionSolverName {
	fn from(value: &LinearRegressionSolver) -> Self {
		match value {
			LinearRegressionSolver::QR => LinearRegressionSolverName::QR,
			LinearRegressionSolver::SVD => LinearRegressionSolverName::SVD,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum RidgeRegressionSolver {
	Cholesky,
	SVD,
}

impl From<&RidgeRegressionSolver> for RidgeRegressionSolverName {
	fn from(value: &RidgeRegressionSolver) -> Self {
		match value {
			RidgeRegressionSolver::Cholesky => {
				RidgeRegressionSolverName::Cholesky
			},
			RidgeRegressionSolver::SVD => RidgeRegressionSolverName::SVD,
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct Config {
	general: GeneralConfig,
	model: Model,
	features: Vec<FeatureConfig>,
}

impl Config {
	pub fn general(&self) -> &GeneralConfig {
		&self.general
	}

	pub fn model(&self) -> &Model {
		&self.model
	}

	pub fn features(&self) -> &Vec<FeatureConfig> {
		&self.features
	}

	pub fn included_features(
		&self
	) -> Result<Vec<FeatureConfig>, Box<dyn Error>> {
		let mut output: Vec<FeatureConfig> = Vec::new();

		for feature in &self.features {
			if feature.include() {
				output.push(*feature);
			}
		}

		Ok(output)
	}

	pub fn model_file_exists(&self) -> bool {
		Path::new(DEFAULT_MODEL_DIR)
			.join(format!("{}.toml", &self.general.output_name))
			.exists()
	}

	pub fn model_dir_exists(&self) -> bool {
		Path::new(DEFAULT_MODEL_DIR).exists()
	}

	pub fn input_data_parent_dir(&self) -> PathBuf {
		Path::new(DEFAULT_DATA_DIR).into()
	}

	pub fn input_data_path(&self) -> PathBuf {
		Path::new(DEFAULT_DATA_DIR).join(&self.general.input_data)
	}

	pub fn output_data_parent_dir(&self) -> PathBuf {
		Path::new(DEFAULT_MODEL_DIR).into()
	}

	pub fn output_data_path(&self) -> PathBuf {
		Path::new(DEFAULT_MODEL_DIR)
			.join(format!("{}.toml", &self.general.output_name))
	}
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
	let args = CliArgs::try_parse()?;
	let config_contents = std::fs::read_to_string(
		Path::new(DEFAULT_TRAIN_CONFIG_DIR).join(args.config),
	)?;
	let config: Config = toml::from_str(&config_contents)?;

	#[cfg(debug_assertions)]
	println!("Successfully read in config!");

	Ok(config)
}

// #[derive(Debug, Clone, ValueEnum)]
// pub enum Dataset {
// 	Synthetic1,
// 	Synthetic2,
// 	Analogous,
// 	#[value(name = "in-lab-single")]
// 	InLabSingle,
// }
