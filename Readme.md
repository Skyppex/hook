# Hook

Hook is a tool which makes it easier to create symlinks by allowing
_destructive_ operations. This might sound scary, but it really just moves files
if it needs to and creates a symlink to where they were before so your other
apps don't break :D

## Installation

- Build from source

Make sure you have `cargo` and `rustc` installed.
Then pull the repo onto your machine.

```pwsh
git clone https://github.com/Skyppex/hook.git
or
gh repo clone Skyppex/hook
```

Navigate to the path containing the `cargo.toml` file.
Run `cargo build --release` and the executable will be dumped in the
`./target/release` folder. From there you have the binary and can execute it from your command line. :D
Add it to your PATH or copy it to a folder already in your PATH to

## Usage

`hook.exe [OPTIONS] --source <SOURCE> --destination <DESTINATION>`

### Options

- `-s`, `--source` <SOURCE> The file path where you wish the real files to be
- `-d`, `--destination` <DESTINATION> The file path where you wish the symlink files to be
- `-f`, `--force` Move files from the destination path to the source path and overwrite if they exist in the source directory
- `-h`, `--help` Print help
- `-V`, `--version` Print version

## Pull Requests & Issues

If you have some functionality you wish to add then make a PR.
If you find a bug or want to discuss something about the tool make an issue out
of it and we can discuss it :D

## License

This tool is licensed under `CC0` so feel free to do whatever you want with it
with no obligation to credit me.
