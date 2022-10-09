---
sidebar_position: 1
---

# Installation Guide

## Linux
The available installation format for linux is .deb.
The guide for this can be found below.
### .deb
- Download the .deb binary installer from the [binary downloads page](/docs/extras/downloads)
- From your terminal, navigate to the downloads folder or where the installed .deb file is.
- Finally, run the following command to install xeeg globally
```bash
sudo apt install ./xeeg-0.1.0.deb
```
(Assuming the installed file is called 'xeeg.deb')


## MacOS
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

## Windows
- Download the executable (.exe) binary from the [binary downloads page](/docs/extras/downloads)

- Run md %USERPROFILE%\xeeg in your terminal and copy the download file to this location.
***If you would like to see what this location is, run `echo %USERPROFILE%\xeeg\xeeg.exe`***
Hence your binary will be located at `%USERPROFILE%\xeeg\xeeg.exe`

- Run control sysdm.cpl, and this will open a dialog menu. In the tab "Advanced" click on "Environment Variables..." and add %USERPROFILE%\xeeg\xeeg.exe to the PATH variable of your account. **Don't edit the global PATH variable**.
You may decide to it at the start or at the end of the PATH value. Windows goes through this in order and will stop as soon as a match is found.