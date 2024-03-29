# tui-chan
An Imageboard Terminal User Interface.
Currently supports only 4chan.

![demo](docs/demo.gif)

## Installation
Download the [latest release][latest-releases]. The binary executable is `tui-chan`. Put it in your PATH so that you can execute it from everywhere.

Then run it from the command line.
```shell
tui-chan
```

You may specify an imageboard name as an argument, the default one is `4chan`.

## Building from source
If your architecture is not supported by the pre-built binaries you can build the application from the source code yourself.
Make sure you have [Rust][rust-installation-url] installed.

```shell
git clone https://github.com/tuqqu/tui-chan.git
cd tui-chan
cargo install --path . # copies binary to /.cargo/bin/

# to uninstall run
cargo uninstall
```

## Controls

Controls can be configured in `~/.config/tui-chan/keybinds.conf`

### Default controls

Press `h` to show / hide help bar to look up controls.
Use `d` to open board or thread and `a` to return to the previous panel.

| Description                                          | Keys                          |
|------------------------------------------------------|-------------------------------|
| Move around                                          | `w`,`a`,`s`,`d`               |
| Move quickly                                         | control + `w`,`a`,`s`,`d`     |
| Toggle help bar                                      | `h`                           |
| Next page                                            | `p`                           |
| Previous page                                        | control + `p`                 |
| Reload page                                          | `r`                           |
| Toggle fullscreen for the selected panel             | `z`                           |
| Copy the direct url to the selected thread or post   | `c`                           |
| Copy the selected post media (image/webm) url        | control + `c`                 |
| Open the selected thread or post in browser          | `o`                           |
| Open the selected post media (image/webm) in browser | control + `o`                 |
| Quit                                                 | `q`                           |

[latest-releases]: https://github.com/tuqqu/tui-chan/releases
[rust-installation-url]: https://www.rust-lang.org/tools/install
