# Structe CLI

## Installing Vlang

### Windows
```sh
git clone https://github.com/vlang/v
cd v
make.bat
# On Windows, open an Administrator shell and run the following.
# NOTE: This is the ONLY command that neeeds to be run from an Administrator shell.
 .\v.exe symlink
```

### MacOS

```sh
git clone https://github.com/vlang/v
cd v
make
sudo ./v symlink
```

## Vscode integration

[Vlang](https://marketplace.visualstudio.com/items?itemName=vlanguage.vscode-vlang)

## Running
This section explains how to run the CLI

### Inline
```sh
$ v run ./src/structe.v
# OR 
$ v run .
```

### Compiling and running

This section explain how to compile and run the executable of v
#### Production
```sh
v --prod ./src/structe.v && ./src/structe
```
#### Development
```sh
v ./src/structe.v && ./src/structe
```