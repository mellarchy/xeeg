---
sidebar_position: 2
---

# Configuration

## How to

Configuring Xeeg to work for your special use case is quick and easy.

By default Xeeg uses a global configuration file that is generated the first time you generate a file. The location of this file differs within different operating systems.

To view the location of this file, run the command:

```bash
xeeg where:global-config
```

This means that you can have a global configuration file for running Xeeg everywhere.

However, you can also have different configuration files per project. To do this, you'll need to generate a config file within the root of your project (assuming you'll be calling xeeg from within the root of your project directory).
To generate a config file, run the command:

```bash
xeeg init
```

This will create a new configuration file at the root of your project.

At any time, you can check the config file being used for the current directory by running the command:

```bash
xeeg which:config
```

:::tip Writing YML Files

[Click here](https://en.wikipedia.org/wiki/YAML) to learn more about YAML

Or visit the [official website](https://yaml.org)

:::


## Configuration Options

The following are the configuration options available


| Name | Description | Type | Default Value |
| ------ | ------ | ------ | ------ |
| stubs_dir | The location of the stub files. (Relative to the current directory of the command line) | string | ./stubs |
| output_dir | The output location of the generated files (Relative to the current directory of the command line) | string | ./generated |
| stub_file_extension | The file extension of the stub files to use. | string | .stub |
| needles | The needle keys that xeeg will find and replace | Map<string, [Needle](/docs/references/needles)> | Refer to the [needles reference](/docs/references/needles) |
| model | Dynamic configuration for each stub file. It is an object where the key is the name of the stub file (without the file extension) and the value is the unique configuration for the stub file. You can specify the following options: `stubs_dir`, `output_dir`, `stub_file_extension` | object | ~ |

Below is an example config file:

```yml title="xeeg.yml"
---
stubs_dir: "./stubs/"
output_dir: "./generated/"
stub_file_extension: ".stub"

models:
  # configuration for stub with filename: action.stub
  # $ xeeg make:action ExampleModel example-model-action.ts
  action:
    stubs_dir: "./stubs/"
    output_dir: "./src/lib/redux/actions"
    stub_file_extension: ".stub"

  # configuration for stub with filename: saga.stub
  # $ xeeg make:saga ExampleModel example-model-saga.ts
  saga:
    stubs_dir: "./stubs/"
    output_dir: "./src/lib/redux/sagas"
    stub_file_extension: ".stub"

  # configuration for stub with filename: reducer.stub
  # $ xeeg make:reducer ExampleModel example-model-reducer.ts
  reducer:
    stubs_dir: "./stubs/"
    output_dir: "./src/lib/redux/reducers"
    stub_file_extension: ".stub"

needles:
  needle: "[NEEDLE]"
  needle_plural: "[NEEDLE_PLURAL]"
  needle_lower: "[NEEDLE_LOWER]"
  needle_lower_plural: "[NEEDLE_LOWER_PLURAL]"
  needle_upper: "[NEEDLE_UPPER]"
  needle_upper_plural: "[NEEDLE_UPPER_PLURAL]"
  needle_title: "[NEEDLE_TITLE]"
  needle_title_plural: "[NEEDLE_TITLE_PLURAL]"
  needle_upper_spaced: "[NEEDLE_UPPER_SPACED]"
  needle_upper_plural_spaced: "[NEEDLE_UPPER_PLURAL_SPACED]"
  needle_lower_spaced: "[NEEDLE_LOWER_SPACED]"
  needle_lower_plural_spaced: "[NEEDLE_LOWER_PLURAL_SPACED]"
  needle_pascal: "[NEEDLE_PASCAL]"
  needle_pascal_plural: "[NEEDLE_PASCAL_PLURAL]"
  needle_camel: "[NEEDLE_CAMEL]"
  needle_camel_plural: "[NEEDLE_CAMEL_PLURAL]"
  needle_upper_camel: "[NEEDLE_UPPER_CAMEL]"
  needle_upper_camel_plural: "[NEEDLE_UPPER_CAMEL_PLURAL]"
  needle_snake: "[NEEDLE_SNAKE]"
  needle_snake_plural: "[NEEDLE_SNAKE_PLURAL]"
  needle_upper_snake: "[NEEDLE_UPPER_SNAKE]"
  needle_upper_snake_plural: "[NEEDLE_UPPER_SNAKE_PLURAL]"
  needle_screaming_snake: "[NEEDLE_SCREAMING_SNAKE]"
  needle_screaming_snake_plural: "[NEEDLE_SCREAMING_SNAKE_PLURAL]"
  needle_kebab: "[NEEDLE_KEBAB]"
  needle_kebab_plural: "[NEEDLE_KEBAB_PLURAL]"
  needle_cobol: "[NEEDLE_COBOL]"
  needle_cobol_plural: "[NEEDLE_COBOL_PLURAL]"
  needle_train: "[NEEDLE_TRAIN]"
  needle_train_plural: "[NEEDLE_TRAIN_PLURAL]"
  needle_flat: "[NEEDLE_FLAT]"
  needle_flat_plural: "[NEEDLE_FLAT_PLURAL]"
  needle_upper_flat: "[NEEDLE_UPPER_FLAT]"
  needle_upper_flat_plural: "[NEEDLE_UPPER_FLAT_PLURAL]"

```
