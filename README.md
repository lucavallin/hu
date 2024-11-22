# A CLI to control Philips Hue lights

`hu` is a CLI to control Philips Hue lights. It is written in Rust and uses the [hue-rs](https://github.com/lucavallin/hue-rs) library to interact with the Hue bridge.

## Build

To build the project, run:

```sh
cargo build [--release]
```

The binary will be placed in the `target/debug` or `target/release` directory.

## Usage

To use the CLI, you need to have a Hue bridge and its IP address. You can find the IP address of your bridge by visiting [discovery.meethue.com](https://discovery.meethue.com/).

Rename the `hu.toml.example` file to `hu.toml` and set the `[bridge].ip` variable to the IP address of your bridge.

The first time you run the CLI, you will need to press the button on the bridge to authenticate. After that, you can use the CLI to control your lights. To initialize the connection to the bridge, run:

```sh
hu init
```

To see the available commands, run:

```sh
hu --help
```
