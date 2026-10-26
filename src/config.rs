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
	splitting_seed: Option<u32>,
	test_size: f32,
}

impl GeneralConfig {
	pub fn input_data_dir(&self) -> &str {
		&self.input_data_dir
	}

	pub fn output_name(&self) -> &str {
		&self.output_name
	}

	pub fn splitting_seed(&self) -> Option<u32> {
		self.splitting_seed
	}

	pub fn test_size(&self) -> f32 {
		self.test_size
	}
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub enum LinearRegressionSolver {
	QR,
	SVD,
}

#[derive(Debug, Deserialize)]
pub enum RidgeRegressionSolver {
	Cholesky,
	SVD,
}

fn default_exp() -> f32 {
	1.0
}

#[derive(Debug, Deserialize)]
pub struct Config {
	general: GeneralConfig,
	model: Model,
	features: HashMap<String, FeatureConfig>,
}

impl Config {
	pub fn general(&self) -> &GeneralConfig {
		&self.general
	}

	pub fn model(&self) -> &Model {
		&self.model
	}

	pub fn features(&self) -> &HashMap<String, FeatureConfig> {
		&self.features
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

#[derive(Debug, Clone, ValueEnum)]
pub enum Dataset {
	Synthetic1,
	Synthetic2,
	Analogous,
	#[value(name = "in-lab-single")]
	InLabSingle,
}
