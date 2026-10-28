use std::error::Error;

use csv::Reader;
use smartcore::{
	linear::{
		elastic_net::{
			ElasticNet,
			ElasticNetParameters,
		},
		lasso::{
			Lasso,
			LassoParameters,
		},
		linear_regression::{
			LinearRegression,
			LinearRegressionParameters,
			LinearRegressionSolverName,
		},
		ridge_regression::{
			RidgeRegression,
			RidgeRegressionParameters,
			RidgeRegressionSolverName,
		},
	},
	metrics::{
		mean_absolute_error,
		mean_squared_error,
		r2,
	},
	model_selection::train_test_split,
};
use toml::to_string_pretty;
use vision_diff::{
	Accuracy,
	CSVRowItem,
	FeatureOutput,
	Output,
	config::{
		Model,
		read_config,
	},
	transform::CSVMatrix,
};

fn main() -> Result<(), Box<dyn Error>> {
	run()
}

fn run() -> Result<(), Box<dyn Error>> {
	let config = read_config()?;

	if !config.model_dir_exists() {
		return Err("Could not find model directory.".into());
	}

	if config.model_file_exists() {
		return Err(
			"Cannot create model: Model already exists. Either delete previous model or change 'output_name' in config.".into()
		);
	}

	let mut csv_reader = Reader::from_path(config.input_data_path())?;
	let mut matrix: CSVMatrix = CSVMatrix(Vec::new());

	for result in csv_reader.deserialize() {
		let row: CSVRowItem = result?;
		matrix.0.push(row);
	}

	let (x, y, included_features) = matrix.transform(&config)?;

	// #[cfg(debug_assertions)]
	// {
	// 	println!("Target values:");
	// 	for target in &y {
	// 		println!("{target}")
	// 	}
	// }

	let (x_train, x_test, y_train, y_test) = train_test_split(
		&x,
		&y,
		config.general().test_size(),
		config.general().shuffle(),
		config.general().splitting_seed(),
	);

	let output_model: Output = match config.model() {
		Model::LinearRegression {
			solver,
		} => {
			let smartcore_solver: LinearRegressionSolverName = solver.into();
			let parameters = LinearRegressionParameters {
				solver: smartcore_solver,
			};
			let model = LinearRegression::fit(&x_train, &y_train, parameters)?;

			let coefficients = model.coefficients();
			let zipped_features: Vec<FeatureOutput> = included_features
				.iter()
				.map(|f| f.clone())
				.zip(coefficients.iter().map(|c| *c))
				.zip(config.features().iter())
				.map(|((col, coef), feat)| (col, coef, *feat).into())
				.collect();

			let intercept = model.intercept();

			let predictions = model.predict(&x_test)?;
			let mae = mean_absolute_error(&y_test, &predictions);
			let mse = mean_squared_error(&y_test, &predictions);
			let r2_score = r2(&y_test, &predictions);

			let accuracy = Accuracy::new(mae, mse, r2_score);

			Output::new(*config.model(), zipped_features, *intercept, accuracy)
		},

		Model::RidgeRegression {
			solver,
			alpha,
			normalize,
		} => {
			let smartcore_solver: RidgeRegressionSolverName = solver.into();
			let parameters = RidgeRegressionParameters {
				solver: smartcore_solver,
				alpha: *alpha,
				normalize: *normalize,
			};
			let model = RidgeRegression::fit(&x_train, &y_train, parameters)?;

			let coefficients = model.coefficients();
			let zipped_features: Vec<FeatureOutput> = included_features
				.iter()
				.map(|f| f.clone())
				.zip(coefficients.iter().map(|c| *c))
				.zip(config.features().iter())
				.map(|((col, coef), feat)| (col, coef, *feat).into())
				.collect();

			let intercept = model.intercept();

			let predictions = model.predict(&x_test)?;
			let mae = mean_absolute_error(&y_test, &predictions);
			let mse = mean_squared_error(&y_test, &predictions);
			let r2_score = r2(&y_test, &predictions);

			let accuracy = Accuracy::new(mae, mse, r2_score);

			Output::new(*config.model(), zipped_features, *intercept, accuracy)
		},

		Model::Lasso {
			alpha,
			normalize,
			tol,
			max_iter,
			fit_intercept,
		} => {
			let parameters = LassoParameters {
				alpha: *alpha as f64,
				normalize: *normalize,
				tol: *tol as f64,
				max_iter: *max_iter as usize,
				fit_intercept: *fit_intercept,
			};

			let model = Lasso::fit(&x_train, &y_train, parameters)?;

			let coefficients = model.coefficients();
			let zipped_features: Vec<FeatureOutput> = included_features
				.iter()
				.map(|f| f.clone())
				.zip(coefficients.iter().map(|c| *c))
				.zip(config.features().iter())
				.map(|((col, coef), feat)| (col, coef, *feat).into())
				.collect();

			let intercept = model.intercept();

			let predictions = model.predict(&x_test)?;
			let mae = mean_absolute_error(&y_test, &predictions);
			let mse = mean_squared_error(&y_test, &predictions);
			let r2_score = r2(&y_test, &predictions);

			let accuracy = Accuracy::new(mae, mse, r2_score);

			Output::new(*config.model(), zipped_features, *intercept, accuracy)
		},

		Model::ElasticNet {
			alpha,
			l1_ratio,
			normalize,
			tol,
			max_iter,
		} => {
			let parameters = ElasticNetParameters {
				alpha: *alpha as f64,
				l1_ratio: *l1_ratio as f64,
				normalize: *normalize,
				tol: *tol as f64,
				max_iter: *max_iter as usize,
			};

			let model = ElasticNet::fit(&x_train, &y_train, parameters)?;

			let coefficients = model.coefficients();
			let zipped_features: Vec<FeatureOutput> = included_features
				.iter()
				.map(|f| f.clone())
				.zip(coefficients.iter().map(|c| *c))
				.zip(config.features().iter())
				.map(|((col, coef), feat)| (col, coef, *feat).into())
				.collect();

			let intercept = model.intercept();

			let predictions = model.predict(&x_test)?;
			let mae = mean_absolute_error(&y_test, &predictions);
			let mse = mean_squared_error(&y_test, &predictions);
			let r2_score = r2(&y_test, &predictions);

			let accuracy = Accuracy::new(mae, mse, r2_score);

			Output::new(*config.model(), zipped_features, *intercept, accuracy)
		},
	};

	let output_string = to_string_pretty(&output_model)?;

	std::fs::write(config.output_data_path(), output_string)?;

	Ok(())
}
