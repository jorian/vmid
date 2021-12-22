use vmid::menu;

// This example builds a simple TCP server with some parameters and some output.
// It then builds a TUI to control the parameters and display the output.

fn main() {
    let mut siv = cursive::default();

    menu::new(&mut siv);

    siv.set_user_data(vmid::Orderbook::new(vmid::Chain::VRSCTEST));

    siv.run();
}
