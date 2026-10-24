use clap::{
	Parser,
	ValueEnum,
};

#[derive(Debug, Clone, ValueEnum)]
enum DatasetChoice {
	#[value(name = "synthetic-1")]
	Synthetic1,
	#[value(name = "synthetic-2")]
	Synthetic2,
	Analogous,
	#[value(name = "in-lab-single")]
	InLabSingle,
}

#[derive(Debug, Clone, ValueEnum)]
enum SigmaChoice {
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
enum TrainingMethodChoice {
    LinearRegression,
    RidgeRegression,
    Lasso,
    ElasticNet,
}

#[derive(Debug, Parser)]
struct TrainArgs {
    #[arg(long)]
    dataset: DatasetChoice,

    #[arg(long)]
    sigma: SigmaChoice,

    #[arg(long)]
    method: TrainingMethodChoice,

    #[arg(long, default_value_t = 1)]
    peak_diff: i8,
    // TODO:
    // for each item in csv row, include one of this.
    // They should all be i8 to limit the exponent.
}
