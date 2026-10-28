use std::{
	collections::HashMap,
	error::Error,
	path::Path,
};

use clap::{
	Parser,
	ValueEnum,
};
use serde::Deserialize;
use smartcore::linear::{
	linear_regression::{
		LinearRegressionParameters,
		LinearRegressionSolverName,
	},
	ridge_regression::{
		RidgeRegressionParameters,
		RidgeRegressionSolverName,
	},
};

use crate::Column;

const DEFAULT_DATA_DIR: &str = "./computed-metrics";
const DEFAULT_MODEL_DIR: &str = "./models";
const DEFAULT_CONFIG_DIR: &str = "./config";

#[derive(Debug, Parser)]
struct CliArgs {
	#[arg(long, short = 'c')]
	config: String,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
	input_data_dir: String,
	output_name: String,
	test_size: f32,
	shuffle: bool,
	splitting_seed: Option<u64>,
}

impl GeneralConfig {
	pub fn input_data_dir(&self) -> &str {
		&self.input_data_dir
	}

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
	include: bool,
	min: Option<f32>,
	max: Option<f32>,

	#[serde(default = "default_exp")]
	exponent: f32,
}

impl FeatureConfig {
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

#[derive(Debug, Deserialize, Copy, Clone)]
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

#[derive(Debug, Deserialize, Copy, Clone)]
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

#[derive(Debug, Deserialize, Copy, Clone)]
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
	features: Vec<(String, FeatureConfig)>,
}

impl Config {
	pub fn general(&self) -> &GeneralConfig {
		&self.general
	}

	pub fn model(&self) -> &Model {
		&self.model
	}

	pub fn features(&self) -> &Vec<(String, FeatureConfig)> {
		&self.features
	}

	pub fn included_features(
		&self
	) -> Result<Vec<(Column, FeatureConfig)>, Box<dyn Error>> {
		let mut output: Vec<(Column, FeatureConfig)> = Vec::new();

		for (feature, feature_config) in &self.features {
			if feature_config.include() {
				output.push((feature.as_str().try_into()?, *feature_config))
			}
		}

		Ok(output)
	}
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
	let args = CliArgs::try_parse()?;
	let config_contents = std::fs::read_to_string(
		Path::new(DEFAULT_CONFIG_DIR).join(args.config),
	)?;
	let config: Config = toml::from_str(&config_contents)?;

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
