# Steam dedicated server

A simple application to install, setup and run dedicated servers for Steam games.

## Supported games:
- <img src="https://static.wikia.nocookie.net/cswikia/images/7/7a/CS2_steam_icon.png/revision/latest?cb=20231020163155" width="12" height="12"> Counter Strike 2

## Requirements:
 - Set up SteamCMD: [Windows tutorial](#steamcmd---windows) / [Linux tutorial](#steamcmd---linux)
 - Ensure the correct ports are open: https://developer.valvesoftware.com/wiki/Source_Dedicated_Server

#### SteamCMD - Windows

Create a folder named "SteamCMD" and add the "steamcmd.exe" inside that folder. You can download the file here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

#### SteamCMD - Linux

Follow the SteamCMD setup here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

## How to run

Run the executable and it will show a menu with the games it currently supports:

![imagem](https://github.com/NFSS10/steam-dedicated-server/assets/22588915/1a25733f-c7c7-4c6b-a3a8-44d6e0e08ce6)

Choose the install option or the run option:

![imagem](https://github.com/NFSS10/steam-dedicated-server/assets/22588915/410132d6-d593-484e-a1ae-7d8672168c80)

- **Run server** - Select this to run the server.

    ![run](https://github.com/NFSS10/steam-dedicated-server/assets/22588915/a852b0ba-ab49-4e0b-bbc1-f4acc7eca8d5)

- **Install server** - Select this to install the server. If the server is already installed, it will start server verification process.

    ![install](https://github.com/NFSS10/steam-dedicated-server/assets/22588915/9a696ee8-f6e9-492c-9c91-4c68fc80bb27)

## Games config:

### <img src="https://static.wikia.nocookie.net/cswikia/images/7/7a/CS2_steam_icon.png/revision/latest?cb=20231020163155" width="14" height="14"> Counter Strike 2

All config files are located in the `resources/games/cs2/` folder:
- `cfgs/` - Contains `.cfg` files that can be loaded by the server.
- `config/` - Contains the server config files, namely:
  - `commands.txt` - The list of commands to be executed when by the server. (Reference: https://developer.valvesoftware.com/wiki/Command_line_options)
  - `exec.txt` - The list of `.cfg`  files located in the `cfgs/` folder to be executed by the server.

## Compatibility

Currently this was only tested in the following platforms:
- Windows 11
- Ubuntu-20.04 on WSL2
