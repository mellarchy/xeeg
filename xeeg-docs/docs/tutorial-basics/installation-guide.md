---
sidebar_position: 1
---

# Installation Guide

Getting the xeeg binary on your filesystem


## Automatic Installation

### Linux

> This is experimental and is currently only tested on Ubuntu 20.04

```bash
curl https://raw.githubusercontent.com/mellarchy/xeeg/main/xeeg-cli/scripts/linux-auto-installation.sh | sudo bash -s -
```



## Manual Installation

### Linux
The available installation format for linux is .deb.
The guide for this can be found below.
#### .deb
- Download the .deb binary installer from the [binary downloads page](/docs/extras/downloads)
- From your terminal, navigate to the downloads folder or where the installed .deb file is.
- Finally, run the following command to install xeeg globally
```bash
sudo apt install ./xeeg-0.1.0.deb
```
(Assuming the installed file is called 'xeeg.deb')


### MacOS
- Create directory ${HOME}/bin by running

```bash
mkdir -p ${HOME}/bin
```

- Save the xeeg binary to directory ${HOME}/bin

- Make the binary executable by running

```bash
chmod 755 ${HOME}/bin/xeeg
```

- Open your shell config file in a text editor. If the file doesn’t exist, create it.
Add the line below to the shell config file, then save it

```bash
export PATH="${HOME}/bin:${PATH}"
```

- Finally, restart your terminal to apply the change. You can verify the binary is on your PATH by running

```bash
command -v xeeg
```

### Windows
- Download the executable (.exe) binary from the [binary downloads page](/docs/extras/downloads)

- Run md %USERPROFILE%\xeeg in your terminal and copy the downloaded file to this location.
***If you would like to see what this location is, run `echo %USERPROFILE%\xeeg\xeeg.exe`***
Hence your binary will be located at `%USERPROFILE%\xeeg\xeeg.exe`

- Run control sysdm.cpl, and this will open a dialog menu. In the tab "Advanced" click on "Environment Variables..." and add `%USERPROFILE%\xeeg` to the PATH variable of your account.
You may decide to place it at the start or at the end of the PATH value. Windows goes through this in order and will stop as soon as a match is found.


### Building From Source

Get the Xeeg binary by building it locally from source

#### Prerequisites
- rustup - An installer for the systems programming language Rust. (Refer to [rustup's installation guide](https://rustup.rs/))
- cargo - The Rust build system. (Refer to [cargo's installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html))

#### Steps

- Clone the project

```bash
git clone https://github.com/mellarchy/xeeg
```

- Navigate to the project directory in your terminal and build and install the project by running the command below

```bash
cargo build --release
```

- This will generate a suitable binary for your OS which can be installed globally.
