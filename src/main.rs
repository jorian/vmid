use color_eyre::Report;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use tracing::info;
use tracing_subscriber::EnvFilter;
use vmid::menu;

fn main() {
    setup().expect("set up of reporting");
    info!("setup succeeded");
    let mut siv = cursive::default();

    menu::new(&mut siv);

    siv.set_user_data(vmid::Data::new());

    siv.add_layer(
        Dialog::new().content(TextView::new("<no chain selected>").with_name("chain_name")),
    );

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
