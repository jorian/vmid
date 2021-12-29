# VMID

A simple TUI that shows the current orderbook of the selected (locally installed) currency.

- [x] only work with chains that have been found on the local system

## Build and run

### MacOS and Linux

All you need is Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
Paste the above command in your terminal and let it do its work.

When it finished, change directory to this repository and run `cargo run >> output.log 2>&1`  
If you see weird characters, make sure to have the `LANG` environment variable set to UTF-8: `LANG=en_US.UTF-8 cargo run >> output.log 2>&1`

### Windows

Coming soon...

### Todo

- [ ] show info on individual offers
- [ ] support id <-> id orders
- [ ] support currency orders

### Wishlist

- [ ] actual trade
- [ ] create new offer
- [ ] create new identity
