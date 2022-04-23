# Sleet

Meant for me but you can use it too.

It writes the packages you install into "$HOME/.config/installer/list.shnaw".

Then, if you ever change distro, or simply delete your whole drive you have it as a backup! Very nice!


NOTE: It doesn't work with all package managers. For instance, it doesn't work with xbps, because it has various commands such as xbps-install or xbps-query 

# Setting up

Go into the file and change the following consts
    
    INSTALLER = [Your package manager]
    INSTALL_ARG = [The argument your package manager takes to install]
    REMOVE_ARG = [Same as INSTALL_ARG, but for removing]
    SEARCH_ARG = [Same as INSTALL_ARG, but for searching]

# Commands:

search: queries the package(s)

install: installs the package(s)

remove: uninstalls the package(s)

recover: installs every package on the list
