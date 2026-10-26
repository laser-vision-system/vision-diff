use smartcore::{
	linalg::basic::matrix::DenseMatrix,
	linear::linear_regression::LinearRegression,
	metrics::mean_squared_error,
	model_selection::train_test_split,
};

pub fn linear_regression(
	x: &DenseMatrix<f32>,
	y: &Vec<f32>,
) -> LinearRegression<f32, f32, DenseMatrix<f32>, Vec<f32>> {
	let (x_train, x_test, y_train, y_test) =
		train_test_split(x, y, 0.2, true, None);

	LinearRegression::fit(&x_train, &y_train, Default::default())?
}
