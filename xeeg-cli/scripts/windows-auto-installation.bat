@echo off


# curl https://raw.githubusercontent.com/mellarchy/xeeg/main/xeeg-cli/scripts/windows-auto-installation.bat && ./windows-auto-installation.bat && del ./windows-auto-installation.bat


REM Download the executable
curl https://raw.githubusercontent.com/mellarchy/xeeg/main/xeeg-cli/downloads/x86_64-pc-windows-gnu/release/xeeg.exe -o C:\temp\xeeg.exe

REM Enusre that the binary was Downloaded to the Right Path
if exist "C:\temp\xeeg.exe" (

	REM Create the destination directory if it does not exist
	if not exist C:\bin md C:\bin

	REM Copy the executable to "C:/bin"
	copy /y C:\temp\xeeg.exe C:\bin\xeeg.exe

	REM Add the value "C:\bin" to the PATH variable
	if "C:\bin" == "%PATH:C:\bin=%" (
		setx PATH "%PATH%;C:\bin"
	) else (
		echo "C:\bin" is already in the PATH variable
	)

	REM Verify the installation
	if exist "C:\bin\xeeg.exe" (
		xeeg -V
	) else (
		echo It seems the installation was not successful
	)

	REM Delete the redundant executable installer file
	del C:\temp\xeeg.exe
)


:end
