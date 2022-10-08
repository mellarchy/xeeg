extern crate confy;

use std::path::Path;
use std::path::PathBuf;
use console::style;
use std::collections::HashMap;
use serde_json::{ json, Value, from_value };


#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
	pub stubs_dir: String,
	pub output_dir: String,
	pub stub_file_extension: String,

	// probably not very important to make these dynamic
	// for each model
	// pub needles: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Needles {
	pub needle: String,
	pub needle_plural: String,
	pub needle_lower: String,
	pub needle_lower_plural: String,
	pub needle_upper: String,
	pub needle_upper_plural: String,

	pub needle_title: String,
	pub needle_title_plural: String,
	pub needle_upper_spaced: String,
	pub needle_upper_plural_spaced: String,
	pub needle_lower_spaced: String,
	pub needle_lower_plural_spaced: String,
	pub needle_pascal: String,
	pub needle_pascal_plural: String,
	pub needle_camel: String,
	pub needle_camel_plural: String,
	pub needle_upper_camel: String,
	pub needle_upper_camel_plural: String,
	pub needle_snake: String,
	pub needle_snake_plural: String,
	pub needle_upper_snake: String,
	pub needle_upper_snake_plural: String,
	pub needle_screaming_snake: String,
	pub needle_screaming_snake_plural: String,
	pub needle_kebab: String,
	pub needle_kebab_plural: String,
	pub needle_cobol: String,
	pub needle_cobol_plural: String,
	pub needle_train: String,
	pub needle_train_plural: String,
	pub needle_flat: String,
	pub needle_flat_plural: String,
	pub needle_upper_flat: String,
	pub needle_upper_flat_plural: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenConfig {
	pub models: Option<HashMap<String, Model>>,
	pub stubs_dir: String,
	pub output_dir: String,
	pub stub_file_extension: String,

	pub needles: Needles,

}

impl Default for GenConfig {
	fn default() -> Self {
		GenConfig {
			models: None,
			stubs_dir: String::from("./stubs/"),
			output_dir: String::from("./generated/"),
			stub_file_extension: String::from(".stub"),
			needles: { Needles {
				needle: String::from("[NEEDLE]"),
				needle_plural: String::from("[NEEDLE_PLURAL]"),
				needle_lower: String::from("[NEEDLE_LOWER]"),
				needle_lower_plural: String::from("[NEEDLE_LOWER_PLURAL]"),
				needle_upper: String::from("[NEEDLE_UPPER]"),
				needle_upper_plural: String::from("[NEEDLE_UPPER_PLURAL]"),

				needle_title: String::from("[NEEDLE_TITLE]"),
				needle_title_plural: String::from("[NEEDLE_TITLE_PLURAL]"),
				needle_upper_spaced: String::from("[NEEDLE_UPPER_SPACED]"),
				needle_upper_plural_spaced: String::from("[NEEDLE_UPPER_PLURAL_SPACED]"),
				needle_lower_spaced: String::from("[NEEDLE_LOWER_SPACED]"),
				needle_lower_plural_spaced: String::from("[NEEDLE_LOWER_PLURAL_SPACED]"),
				needle_pascal: String::from("[NEEDLE_PASCAL]"),
				needle_pascal_plural: String::from("[NEEDLE_PASCAL_PLURAL]"),
				needle_camel: String::from("[NEEDLE_CAMEL]"),
				needle_camel_plural: String::from("[NEEDLE_CAMEL_PLURAL]"),
				needle_upper_camel: String::from("[NEEDLE_UPPER_CAMEL]"),
				needle_upper_camel_plural: String::from("[NEEDLE_UPPER_CAMEL_PLURAL]"),
				needle_snake: String::from("[NEEDLE_SNAKE]"),
				needle_snake_plural: String::from("[NEEDLE_SNAKE_PLURAL]"),
				needle_upper_snake: String::from("[NEEDLE_UPPER_SNAKE]"),
				needle_upper_snake_plural: String::from("[NEEDLE_UPPER_SNAKE_PLURAL]"),
				needle_screaming_snake: String::from("[NEEDLE_SCREAMING_SNAKE]"),
				needle_screaming_snake_plural: String::from("[NEEDLE_SCREAMING_SNAKE_PLURAL]"),
				needle_kebab: String::from("[NEEDLE_KEBAB]"),
				needle_kebab_plural: String::from("[NEEDLE_KEBAB_PLURAL]"),
				needle_cobol: String::from("[NEEDLE_COBOL]"),
				needle_cobol_plural: String::from("[NEEDLE_COBOL_PLURAL]"),
				needle_train: String::from("[NEEDLE_TRAIN]"),
				needle_train_plural: String::from("[NEEDLE_TRAIN_PLURAL]"),
				needle_flat: String::from("[NEEDLE_FLAT]"),
				needle_flat_plural: String::from("[NEEDLE_FLAT_PLURAL]"),
				needle_upper_flat: String::from("[NEEDLE_UPPER_FLAT]"),
				needle_upper_flat_plural: String::from("[NEEDLE_UPPER_FLAT_PLURAL]"),
			}
		},
	}
}
}

pub fn get_project_config() -> Result<GenConfig, confy::ConfyError> {
	let config_path = Path::new("xeeg.yml");
	// let file: PathBuf;
	let cfg: GenConfig;
	if config_path.exists() {
		cfg = confy::load_path(&config_path)?;
		// file = config_path.to_path_buf();
	} else {
		cfg = confy::load("xeeg", None)?;
		// file = confy::get_configuration_file_path("xeeg", None)?;
	}

	// println!("Using config file at: {:#?}", file);

	return Ok(cfg);

}

pub fn init_current_dir() -> Result<bool, confy::ConfyError> {
	let cfg: GenConfig = Default::default();
	let config_path = Path::new("xeeg.yml");
	if config_path.exists() {
		println!("{}", style("A config file already exists in the current directory").red());
		return Ok(false);
	}

	confy::store_path(&config_path, cfg)?;
	if config_path.exists() {
		println!("{}", style("✅ Config file created in the current directory").green());
	} else {
		println!("{}", style("Failed to create config file").red());
	}

	return Ok(true);
}

pub fn merge_model_config(cfg: &GenConfig, make_command_1: String) -> GenConfig {
	let mut cfg_json = json!(&cfg);
	let model = &cfg.models.as_ref().unwrap()[&make_command_1];
	let model_json = json!(&model);

	fn merge(a: &mut Value, b: Value) {
		match (a, b) {
			(a @ &mut Value::Object(_), Value::Object(b)) => {
				let a = a.as_object_mut().unwrap();
				for (k, v) in b {
					merge(a.entry(k).or_insert(Value::Null), v);
				}
			}
			(a, b) => *a = b,
		}

	}
	// cfg_json gets modified below
	merge(&mut cfg_json, model_json);

	let new_cfg: GenConfig = from_value(cfg_json).unwrap();

	return new_cfg;
}


pub fn print_current_config_dir() -> Result<(), confy::ConfyError> {
	let config_path = Path::new("xeeg.yml");
	let file_path: PathBuf;

	if config_path.exists() {
		file_path = config_path.to_path_buf();
	} else {
		file_path = confy::get_configuration_file_path("xeeg", None)?;
	}

	println!("{}", file_path.display());

	Ok(())
}

pub fn print_global_config_dir() -> Result<(), confy::ConfyError> {
	let file_path: PathBuf = confy::get_configuration_file_path("xeeg", None)?;

	println!("{}", file_path.display());

	Ok(())
}
