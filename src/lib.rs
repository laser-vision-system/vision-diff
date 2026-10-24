use clap::ValueEnum;

pub mod metric_extraction;
pub mod mapping;
pub mod dataset;
pub mod command;
pub mod path;

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

#[derive(Debug, Clone, ValueEnum)]
pub enum TrainingMethod {
	LinearRegression,
	RidgeRegression,
	Lasso,
	ElasticNet,
}
