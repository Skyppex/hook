# Hook

Hook is a tool which makes it easier to create symlinks by allowing *destructive* operations. This might sound scary, but it really just moves files if it needs to and creates a symlink to where they were before so your other apps don't break :D

## Installation

- Using scoop
```pwsh
scoop bucket add sky-bucket https://github.com/skyppex/sky-bucket
scoop install skyppex/hook
```

- Build it your self
Make sure you have `cargo` and `rustc` installed.
Then pull the repo onto your machine.
```pwsh
git clone https://github.com/Skyppex/hook.git
```
Navigate to the path containing the `cargo.toml` file.
Make sure to take a look at the code so you know its not malware (i had some problems with my anti malware software netralizing the binary when i ran it).
Run `cargo build --release` and the executable will be dumped in the `./target/release` folder. From there you have the binary and can execute it from your command line. :D
It's recommended to add it to your PATH or copy it to a folder already in your PATH.

## Usage

`hook.exe [OPTIONS] --source <SOURCE> --destination <DESTINATION>`

### Options
- `-s`, `--source` <SOURCE>            The file path where you wish the real files to be
- `-d`, `--destination` <DESTINATION>  The file path where you wish the symlink files to be
- `-f`, `--force`                      Move files from the destination path to the source path and overwrite if they exist in the source directory
- `-h`, `--help`                       Print help
- `-V`, `--version`                    Print version

## Pull Requests & Issues

If you have some functionality you wish to add then make a PR.
If you find a bug or want to discuss something about the tool make an issue out of it and we can discuss it :D

## License

This tool is licensed under `CC0` so feel free to do whatever you want with it with no obligation to credit me.
