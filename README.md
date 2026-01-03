<div align="center">

# lshw-tui

This is a TUI wrapper for the [lshw](https://github.com/lyonel/lshw) command utility. 
It provides a UI for quickly browsing a tree of Linux hardware devices.
You can easily navigate the tree-style UI with Vim-like key bindings.

</div>

<div align="center">

![screenshot](./assets/screenshot.png)

</div>

## Usage

This tool provides simple key bindings similar to `less` pager or Vim.

| Key | Action |
|-----|--------|
| `k` or Up | Up |
| `j` or Down | Down |
| `h` or Left or Backspace | Close Node |
| `l` or Right or Enter | Open Node |
| `q` | Quit |

The help for the commands is as follows:

```console
Usage: lshw-tui [OPTIONS]

Options:
  -c, --command <COMMAND>  Command name of 'lshw' [default: lshw]
  -s, --sanitize           Sanitize sensitive values like serial numbers, etc
  -h, --help               Print help
```


## Requirements

Requires [lshw](https://github.com/lyonel/lshw), testing on version `B.02.20`.

## Build / Install

To build from source, run `cargo build --release`.
To install, run `cargo install --path=.`

## License

See [LICENSE](./LICENSE)

## Thanks

- [tokio](https://tokio.rs) - High-performance async runtime for Rust 🗼
- [ratatui](https://ratatui.rs) - Awesome TUI rendering library for Rust 🐀
- [tui-tree-widget](https://github.com/EdJoPaTo/tui-rs-tree-widget) - Tree widget for ratatui 🌲


## Author

sheepla
