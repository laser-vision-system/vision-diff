use std::{
	collections::HashMap,
	error::Error,
};

use smartcore::linalg::basic::matrix::DenseMatrix;

use crate::{
	CSVRowItem,
	Column,
	config::{
		Config,
		FeatureConfig,
	},
};

pub struct CSVMatrix(pub Vec<CSVRowItem>);

impl CSVMatrix {
	pub fn transform(
		self,
		config: &Config,
	) -> Result<(DenseMatrix<f32>, Vec<f32>, Vec<Column>), Box<dyn Error>> {
		let mut matrix: Vec<Vec<f32>> = Vec::new();
		let included_features = config.included_features()?;
		let mut target: Vec<f32> = Vec::new();

		for row in self.0 {
			if !all_features_within_bounds(&row, config)? {
				continue;
			}

			// Create the target (known deviation) value for this row
			target.push(
				row.get(&Column::Intensity)
					/ row.get(&Column::SetpointIntensity),
			);

			let mut new_row: Vec<f32> = Vec::new();

			for (feature, feature_config) in &included_features {
				new_row.push(row.get(feature).powf(feature_config.exponent()))
			}

			matrix.push(new_row);
		}

		let matrix = DenseMatrix::from_2d_vec(&matrix)?;

		let included_features: Vec<Column> = included_features
			.into_iter()
			.map(|(column, _)| column)
			.collect();

		Ok((matrix, target, included_features))
	}
}

fn all_features_within_bounds(
	row: &CSVRowItem,
	config: &Config,
) -> Result<bool, Box<dyn Error>> {
	if 21 != config.features().len() {
		return Err(Box::new(LengthMismatch(format!(
			"There should always be 21 features!\nNumber of features found: {}",
			config.features().iter().len()
		))));
	} else {
		let mut valid = true;
		for (feature, feature_config) in config.features() {
			let column: Column = feature.as_str().try_into()?;
			let value = row.get(&column);
			let min = feature_config.min();
			let max = feature_config.max();

			valid = valid && within_bounds(value, min, max);

			if !valid {
				break;
			}
		}
		Ok(valid)
	}
}

fn within_bounds(
	value: f32,
	min: Option<f32>,
	max: Option<f32>,
) -> bool {
	min.is_none_or(|min| value >= min) && max.is_none_or(|max| value <= max)
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
