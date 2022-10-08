#![crate_type = "bin"]

#[macro_use]
extern crate serde_derive;

use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::{Context};
use std::io::prelude::*;
extern crate inflector;
use inflector::Inflector;
use console::style;
use std::io::Error;
use std::fs::{ read_to_string, File, create_dir_all };
use convert_case::{ Case, Casing };

mod config;

#[derive(StructOpt)]
struct Cli {
	command: String,
	model: Option<String>,
	output_file_name: Option<String>,

	#[structopt(name = "print", short="p", long = "print", help = "Prints the output to the console instead of saving it to a file")]
	print: bool,

	#[structopt(name = "overwrite-existing", short="o", long = "overwrite-existing", help = "Overwrite the output file if it already exists")]
	overwrite_existing: bool,
}



fn main() -> Result<(), Error> {
	let args = Cli::from_args();
	let command = args.command;
	// let model = args.model.unwrap();
	let model: String;
	if args.model == None {
		model = String::from("unknown");
	} else {
		model = args.model.unwrap();
	}
	// let output_file_name = args.output_file_name.unwrap();

	let command_parts = command.split(":").collect::<Vec<&str>>();

	Ok(
		match command_parts[0] {
			"make" => {
				let output_file_name: String;
				if args.print == false {
					if args.output_file_name == None {
						panic!("You forgot to specify an output file name");
					} else {
						output_file_name = args.output_file_name.unwrap();
					}
				} else {
					output_file_name = String::from("");
				}

				let mut cfg = config::get_project_config().unwrap();
				let mut stub_file_name = String::from(command_parts[1]);

				// TODO: Fix below
				if !&cfg.models.is_none() {
					let cfg_models = &cfg.models.as_ref().unwrap();

					if cfg_models.contains_key(command_parts[1]) {
						cfg = config::merge_model_config(&cfg, String::from(command_parts[1]));
					}
				}

				stub_file_name.push_str(&cfg.stub_file_extension);

				make_file(stub_file_name, model, output_file_name, cfg, args.print, args.overwrite_existing);
			},
			"init" => {
				config::init_current_dir().expect("Failed to init config file in the current directory");
			},
			"which" => {
				match command_parts[1] {
					"config" => {
						config::print_current_config_dir().expect("Unable to find config path");
					},
					_ => panic!("There is no command with name `{}` in the `{}` namespace", command_parts[1], command_parts[0]),
				}
			},
			"where" => {
				match command_parts[1] {
					"global-config" => {
						config::print_global_config_dir().expect("Unable to find global config path");
					},
					_ => panic!("There is no command with name `{}` in the `{}` namespace", command_parts[1], command_parts[0]),
				}
			},
			_ => panic!("Undefined command `{}`", command_parts[0]),
		}
	)
}

fn make_file(stub_file_name: String, model: String, output_file_name: String, cfg: config::GenConfig, print: bool, overwrite_existing: bool) {
	let mut output_path = PathBuf::new();
	let mut action_output_path = String::from(&cfg.output_dir);

	if print == false {
		println!("{}", style("Generating file").cyan());

		create_dir_all(&action_output_path).unwrap();

		action_output_path = action_output_path + &output_file_name;
		// if output_file_name.chars().nth(0) == Some('/') {
			// } else {
				// action_output_path = action_output_path + "/" + &output_file_name;
				// }

				output_path.push(&action_output_path);
			}

			let mut path = PathBuf::new();
			path.push(&cfg.stubs_dir);
			path.push(stub_file_name);



			let replacement = String::from(model);

			process_file(ProcessPayload {
				replacement,
				path,
				output_path,
				cfg,
				print,
				overwrite_existing,
			});

			if print == false {
				println!("{}", style(format!("✅ File generated: {}", action_output_path)).green());
			}
		}


		struct ProcessPayload {
			replacement: String,
			path: std::path::PathBuf,
			output_path: std::path::PathBuf,
			cfg: config::GenConfig,
			print: bool,
			overwrite_existing: bool,
		}

		fn process_file(args: ProcessPayload) {
			let path = &args.path;

			let needle = &args.cfg.needles.needle;
			let needle_plural = &args.cfg.needles.needle_plural;
			let needle_lower = &args.cfg.needles.needle_lower;
			let needle_lower_plural = &args.cfg.needles.needle_lower_plural;
			let needle_upper = &args.cfg.needles.needle_upper;
			let needle_upper_plural = &args.cfg.needles.needle_upper_plural;

			let needle_title = &args.cfg.needles.needle_title;
			let needle_title_plural = &args.cfg.needles.needle_title_plural;
			let needle_upper_spaced = &args.cfg.needles.needle_upper_spaced;
			let needle_upper_plural_spaced = &args.cfg.needles.needle_upper_plural_spaced;
			let needle_lower_spaced = &args.cfg.needles.needle_lower_spaced;
			let needle_lower_plural_spaced = &args.cfg.needles.needle_lower_plural_spaced;
			let needle_pascal = &args.cfg.needles.needle_pascal;
			let needle_pascal_plural = &args.cfg.needles.needle_pascal_plural;
			let needle_camel = &args.cfg.needles.needle_camel;
			let needle_camel_plural = &args.cfg.needles.needle_camel_plural;
			let needle_upper_camel = &args.cfg.needles.needle_upper_camel;
			let needle_upper_camel_plural = &args.cfg.needles.needle_upper_camel_plural;
			let needle_snake = &args.cfg.needles.needle_snake;
			let needle_snake_plural = &args.cfg.needles.needle_snake_plural;
			let needle_upper_snake = &args.cfg.needles.needle_upper_snake;
			let needle_upper_snake_plural = &args.cfg.needles.needle_upper_snake_plural;
			let needle_screaming_snake = &args.cfg.needles.needle_screaming_snake;
			let needle_screaming_snake_plural = &args.cfg.needles.needle_screaming_snake_plural;
			let needle_kebab = &args.cfg.needles.needle_kebab;
			let needle_kebab_plural = &args.cfg.needles.needle_kebab_plural;
			let needle_cobol = &args.cfg.needles.needle_cobol;
			let needle_cobol_plural = &args.cfg.needles.needle_cobol_plural;
			let needle_train = &args.cfg.needles.needle_train;
			let needle_train_plural = &args.cfg.needles.needle_train_plural;
			let needle_flat = &args.cfg.needles.needle_flat;
			let needle_flat_plural = &args.cfg.needles.needle_flat_plural;
			let needle_upper_flat = &args.cfg.needles.needle_upper_flat;
			let needle_upper_flat_plural = &args.cfg.needles.needle_upper_flat_plural;



			let replacement = &args.replacement;
			let replacement_plural = &replacement.to_plural();

			let replacement_upper = &replacement.to_ascii_uppercase();
			let replacement_upper_plural = &replacement.to_plural().to_ascii_uppercase();

			let replacement_lower = &replacement.to_lowercase();
			let replacement_lower_plural = &replacement.to_plural().to_lowercase();

			// new
			// TODO: needles below
			// let replacement_spaced = &replacement.to_plural();
			// let replacement_plural_spaced = &replacement.to_plural();

			let replacement_title = &replacement.to_case(Case::Title);
			let replacement_title_plural = &replacement.to_plural().to_case(Case::Title);

			let replacement_upper_spaced = &replacement.to_case(Case::Upper);
			let replacement_upper_plural_spaced = &replacement.to_plural().to_case(Case::Upper);

			let replacement_lower_spaced = &replacement.to_case(Case::Lower);
			let replacement_lower_plural_spaced = &replacement.to_plural().to_case(Case::Lower);

			let replacement_pascal = &replacement.to_case(Case::Pascal);
			let replacement_pascal_plural = &replacement.to_plural().to_case(Case::Pascal);

			let replacement_camel = &replacement.to_case(Case::Camel);
			let replacement_camel_plural = &replacement.to_plural().to_case(Case::Camel);

			let replacement_upper_camel = &replacement.to_case(Case::UpperCamel);
			let replacement_upper_camel_plural = &replacement.to_plural().to_case(Case::UpperCamel);

			let replacement_snake = &replacement.to_case(Case::Snake);
			let replacement_snake_plural = &replacement.to_plural().to_case(Case::Snake);

			let replacement_upper_snake = &replacement.to_case(Case::UpperSnake);
			let replacement_upper_snake_plural = &replacement.to_plural().to_case(Case::UpperSnake);

			let replacement_screaming_snake = &replacement.to_case(Case::ScreamingSnake);
			let replacement_screaming_snake_plural = &replacement.to_plural().to_case(Case::ScreamingSnake);

			let replacement_kebab = &replacement.to_case(Case::Kebab);
			let replacement_kebab_plural = &replacement.to_plural().to_case(Case::Kebab);

			let replacement_cobol = &replacement.to_case(Case::Cobol);
			let replacement_cobol_plural = &replacement.to_plural().to_case(Case::Cobol);

			let replacement_train = &replacement.to_case(Case::Train);
			let replacement_train_plural = &replacement.to_plural().to_case(Case::Train);

			let replacement_flat = &replacement.to_case(Case::Flat);
			let replacement_flat_plural = &replacement.to_plural().to_case(Case::Flat);

			let replacement_upper_flat = &replacement.to_case(Case::UpperFlat);
			let replacement_upper_flat_plural = &replacement.to_plural().to_case(Case::UpperFlat);
			// end new





			let mut haystack = read_to_string(path)
			.with_context(|| "Unable to read file. It looks like the stub file does not exist.").unwrap();

			haystack = haystack.replace(needle, replacement);
			haystack = haystack.replace(needle_plural, replacement_plural);
			haystack = haystack.replace(needle_lower, replacement_lower);
			haystack = haystack.replace(needle_lower_plural, replacement_lower_plural);
			haystack = haystack.replace(needle_upper, replacement_upper);
			haystack = haystack.replace(needle_upper_plural, replacement_upper_plural);

			haystack = haystack.replace(needle_title, replacement_title);
			haystack = haystack.replace(needle_title_plural, replacement_title_plural);
			haystack = haystack.replace(needle_upper_spaced, replacement_upper_spaced);
			haystack = haystack.replace(needle_upper_plural_spaced, replacement_upper_plural_spaced);
			haystack = haystack.replace(needle_lower_spaced, replacement_lower_spaced);
			haystack = haystack.replace(needle_lower_plural_spaced, replacement_lower_plural_spaced);
			haystack = haystack.replace(needle_pascal, replacement_pascal);
			haystack = haystack.replace(needle_pascal_plural, replacement_pascal_plural);
			haystack = haystack.replace(needle_camel, replacement_camel);
			haystack = haystack.replace(needle_camel_plural, replacement_camel_plural);
			haystack = haystack.replace(needle_upper_camel, replacement_upper_camel);
			haystack = haystack.replace(needle_upper_camel_plural, replacement_upper_camel_plural);
			haystack = haystack.replace(needle_snake, replacement_snake);
			haystack = haystack.replace(needle_snake_plural, replacement_snake_plural);
			haystack = haystack.replace(needle_upper_snake, replacement_upper_snake);
			haystack = haystack.replace(needle_upper_snake_plural, replacement_upper_snake_plural);
			haystack = haystack.replace(needle_screaming_snake, replacement_screaming_snake);
			haystack = haystack.replace(needle_screaming_snake_plural, replacement_screaming_snake_plural);
			haystack = haystack.replace(needle_kebab, replacement_kebab);
			haystack = haystack.replace(needle_kebab_plural, replacement_kebab_plural);
			haystack = haystack.replace(needle_cobol, replacement_cobol);
			haystack = haystack.replace(needle_cobol_plural, replacement_cobol_plural);
			haystack = haystack.replace(needle_train, replacement_train);
			haystack = haystack.replace(needle_train_plural, replacement_train_plural);
			haystack = haystack.replace(needle_flat, replacement_flat);
			haystack = haystack.replace(needle_flat_plural, replacement_flat_plural);
			haystack = haystack.replace(needle_upper_flat, replacement_upper_flat);
			haystack = haystack.replace(needle_upper_flat_plural, replacement_upper_flat_plural);


			if args.print == false {
				if args.output_path.exists() && args.overwrite_existing == false {
					panic!("The file at the output path already exists");
				}

				let file = File::create(args.output_path).with_context(|| "Unable to create file");
				file
				.unwrap()
				.write_all(haystack.as_bytes())
				.with_context(|| "Unable to write data")
				.expect("Unable to write data");
			} else {
				println!("{}", haystack);
			}
		}
