#!/bin/bash

# download the binary
echo "$(tput setaf 4)$(tput setab 0)STEP 1:: Downloading Xeeg Binary (curl)"
if curl https://raw.githubusercontent.com/mellarchy/xeeg/main/xeeg-cli/downloads/xeeg_0.1.0_amd64.deb -L -o xeeg.deb ; then
	sleep 2
	# install the binary
	echo "$(tput setaf 4)$(tput setab 0)STEP 2:: Installing Xeeg Binary"
	if sudo apt install ./xeeg.deb ; then
		sleep 2
		echo "$(tput setaf 4)$(tput setab 0)STEP 3:: Running `xeeg -V`  to Verify The Installation"
		if xeeg -V ; then
			echo "$(tput setaf 4)$(tput setab 0)Installation Complete"
			echo "$(tput setaf 4)$(tput setab 0)STEP 4:: Deleting Redundant Binary Installer File"
			if rm ./xeeg.deb ; then
				echo "✅";
			fi
		fi
	else
		echo "$(tput setaf 1) $(tput setab 0)Failed to install the binary"
	fi
else
	echo "$(tput setaf 1) $(tput setab 0)Failed to download the binary"
fi
