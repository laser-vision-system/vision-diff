use std::error::Error;

use smartcore::linalg::basic::matrix::DenseMatrix;

use crate::{
	config::TrainConfig,
	types::{
		CSVRowItem,
		Column,
	},
};

pub struct CSVMatrix(pub Vec<CSVRowItem>);

impl CSVMatrix {
	pub fn transform(
		self,
		config: &TrainConfig,
	) -> Result<(DenseMatrix<f32>, Vec<f32>, Vec<Column>), Box<dyn Error>> {
		let mut matrix: Vec<Vec<f32>> = Vec::new();
		let included_features = config.included_features()?;
		let mut target: Vec<f32> = Vec::new();

		for row in self.0 {
			// A setpoint intensity of zero does not provide a valid frame of
			// reference. Also avoids division by zero.
			if row
				.get(&Column::SetpointIntensity)
				.expect("Setpoint intensity should always exists")
				== 0.0
			{
				continue;
			}

			if !all_features_within_bounds(&row, config)? {
				continue;
			}

			// Create the target (known deviation) value for this row
			target.push(
				row.get(&Column::Intensity)
					.expect("Iintensity should always exists")
					/ row
						.get(&Column::SetpointIntensity)
						.expect("Setpoint intensity should always exists"),
			);

			let mut new_row: Vec<f32> = Vec::new();

			for feature in &included_features {
				let field = match row.get(feature.column()) {
					Some(value) => value.powf(feature.exponent()),
					None => 0.0,
				};

				new_row.push(field)
			}

			matrix.push(new_row);
		}

		let matrix = DenseMatrix::from_2d_vec(&matrix)?;

		let included_features: Vec<Column> =
			included_features.into_iter().map(|f| *f.column()).collect();

		Ok((matrix, target, included_features))
	}
}

fn all_features_within_bounds(
	row: &CSVRowItem,
	config: &TrainConfig,
) -> Result<bool, Box<dyn Error>> {
	if 21 != config.features().len() {
		return Err(Box::new(LengthMismatch(format!(
			"There should always be 21 features!\nNumber of features found: {}",
			config.features().iter().len()
		))));
	} else {
		let mut valid = true;
		for feature in config.features() {
			let value = row.get(feature.column());
			let min = feature.min();
			let max = feature.max();

			valid = valid && within_bounds(value, min, max);

			if !valid {
				break;
			}
		}
		Ok(valid)
	}
}

pub fn within_bounds(
	value: Option<f32>,
	min: Option<f32>,
	max: Option<f32>,
) -> bool {
	min.is_none_or(|min| value.is_none_or(|value| value >= min))
		&& max.is_none_or(|max| value.is_none_or(|value| value >= max))
}

#[derive(Debug)]
struct LengthMismatch(String);

impl std::fmt::Display for LengthMismatch {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl Error for LengthMismatch {
}
