# Steam dedicated server

A simple application to install, setup and run dedicated servers for Steam games.

## Supported games:
- Counter Strike 2

## Requirements:
 - Set up SteamCMD: [Windows tutorial](#steamcmd---windows)/[Linux tutorial](#steamcmd---linux)
 - Ensure the correct ports are open: https://developer.valvesoftware.com/wiki/Source_Dedicated_Server

#### SteamCMD - Windows

Create a folder named "SteamCMD" and add the "steamcmd.exe" inside that folder. You can download the file here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

#### SteamCMD - Linux

Follow the SteamCMD setup here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

## Games config:

### Counter Strike 2

All config files are located in the `resources/games/cs2/` folder:
- `cfgs/` - Contains `.cfg` files that can be loaded by the server.
- `config/` - Contains the server config files, namely:
  - `commands.txt` - The list of commands to be executed when by the server. (Reference: https://developer.valvesoftware.com/wiki/Command_line_options)
  - `exec.txt` - The list of `.cfg`  files located in the `cfgs/` folder to be executed by the server.

## Compatibility

Currently this was only tested in the following platforms:
- Windows 11
- Ubuntu-20.04 on WSL2
