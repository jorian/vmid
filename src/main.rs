use color_eyre::Report;
use std::sync::{Arc, Mutex};
use tracing::*;
use tracing_subscriber::EnvFilter;
use vmid::views::*;
fn main() {
    setup().expect("set up of reporting");
    info!("setup succeeded");
    let mut siv = cursive::default();

    let data = Arc::new(Mutex::new(vmid::data::Data::new()));
    menu::new(&mut siv, data.clone());

    siv.set_user_data(data.clone());

    siv.add_layer(offers::new());

    siv.run();
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
