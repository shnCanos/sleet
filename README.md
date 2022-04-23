# Sleet

Meant for me, it is public for easier cloning, but you can use it too.

What it does is pretty simple, although it doesn't work with all package managers, such as xbps:

Go into the file and change the consts
    INSTALLER = [Your package manager]
    INSTALL_ARG = [The argument your package manager takes to install]
    REMOVE_ARG = [Same as INSTALL_ARG, but for removing]
    SEARCH_ARG = [Same as INSTALL_ARG, but for searching]
    LIST = You don't need to change this one

It writes the apps you install manually into list.shnaw which is stored in $HOME/.config/installer/list.shnaw Then, if you ever change distro, or simply delete your whole drive you have it as a backup! Very nice!

In addition it can also search!
