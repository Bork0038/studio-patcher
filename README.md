# studio-patcher
It patches studio or something.

# content 
- [screenshots](#screenshots)
- [installation](#installation)
- [building from source](#building-from-source)
  - [dependencies](#dependencies)
  - [running program](#running-program)
  - [buidling installer](#buidling-installer)
- [patches](#patches)
  - [internal studio](#internal-studio)
  - [extended explorer](#extended-explorer)

# screenshots
![alt text](https://raw.githubusercontent.com/Bork0038/studio-patcher/main/assets/pic.png)

# installation
Run the installer from the releases page.

# building from source
## dependencies
 - [Node.js](https://nodejs.org/en)
 - [Rust](https://www.rust-lang.org/)
 - [pNPM](https://pnpm.io/)

## running program
1. ```pnpm i```

2. ```cargo tauri dev```

## buidling installer
The installer will be written to /src-tauri/target/release/bundle
1. ```pnpm i```
2. ```cargo tauri build```
<br>



# patches

## internal studio
The internal studio patch enables Roblox's Internal Studio mode. It provides access to many restricted features that exist within Studio.

## extended explorer
The extend explorer patch shows all hidden properties in the Property View, including hidden and restricted properties.
### example
![alt text](https://raw.githubusercontent.com/Bork0038/studio-patcher/main/assets/extended-explorer.png)
