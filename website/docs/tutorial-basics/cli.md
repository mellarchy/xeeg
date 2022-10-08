---
sidebar_position: 3
---

# CLI Usage

The xeeg cli is what you'll be using to generate files.

Documented below are the available commands.

**Note:** Each command is only available in the `xeeg` namespace


| Command | Description | Arguments | Flags | Example Usage |
| ------ | ------ | ------ | ------ | ------ |
| make:[MODEL] | The command used to generate files based on the value of [MODEL] | `<model>` `<output-file-name>` | `--print (-p)`:  Prints the output instead of dumping it to an output file <br /> `--overwrite-existing (-o)`: Overwrite the output file if it already exists | `xeeg make:model Example output-file.php` |
| init | Generates a configuration file the current directory | ~ | ~ | `xeeg init` |
| which:config | Prints the path to the current configuration file being used. | ~ | ~ | xeeg which:config |
| where:global-config | Prints the path to the global configuration file. | ~ | ~ | xeeg where:global-config |
