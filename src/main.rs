use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use vmid::menu;

// This example builds a simple TCP server with some parameters and some output.
// It then builds a TUI to control the parameters and display the output.

fn main() {
    setup().expect("set up of reporting");
    info!("setup succeeded");
    let mut siv = cursive::default();

    menu::new(&mut siv);

    siv.set_user_data(vmid::Orderbook::new(vmid::Chain::VRSCTEST));

    siv.run();
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
