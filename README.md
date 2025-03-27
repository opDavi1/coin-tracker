# Coin Tracker

A free and open-source app for keeping track of a coin collection and other numismatics.
See the included LICENSE.md file for more information.

## Development
To work on Coin Tracker or build it from source, first make sure to have [Node.js](https://nodejs.org/en/download), as well as [Rust and Cargo](https://rustup.rs/) installed on your computer.
After cloning this repo, make sure to Run the following command to install all of the npm dependencies.
```shell
npm i
```


### Building from source
Run the following command to build Coin Tracker from source for your current operating system / architecture.
```shell
npm run tauri build
```
The binary will be in `./target/release/coin-tracker`. If you're on Windows this may end with a `.exe` extention.


### Contributing
To compile and run the dev build, run the following command.
```shell
npm tauri run dev
```

## Contact
For any questions, comments, concerns, and scathing criticisms, please make an issue on github or contact me directly on discord (@opdavi1)
