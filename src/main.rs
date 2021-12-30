use color_eyre::Report;
use std::sync::{Arc, Mutex};
use tracing::*;
use tracing_subscriber::EnvFilter;
use vmid::*;
fn main() {
    setup().expect("setup of reporting");

    let mut siv = cursive::default();

    let data = Arc::new(Mutex::new(vmid::data::Data::new()));
    views::menu::new(&mut siv, data.clone());

    siv.set_user_data(data.clone());

    siv.add_layer(views::offers::new(
        data.lock().unwrap().active_chain.name.clone(),
    ));

    siv.run();
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "cursive_core=info,vrsc-rpc=info,vmid=debug")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("tracing setup succeeded");

    Ok(())
}
