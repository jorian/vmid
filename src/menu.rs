use crate::data::Data;
use cursive::menu;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::Cursive;
use std::sync::{Arc, Mutex};
use tracing::info;

pub fn new(siv: &mut Cursive, data: Arc<Mutex<Data>>) {
    let data_clone = Arc::clone(&data);

    let menutree = cursive::menu::MenuTree::new().with(move |tree| {
        for chain in data_clone.lock().unwrap().local_chains.iter() {
            let chain = Arc::clone(chain);
            let name = chain.name.clone();

            tree.add_leaf(name, move |s| {
                s.with_user_data(|data: &mut Arc<Mutex<Data>>| {
                    data.lock().unwrap().active_chain = chain.clone();
                    info!("{:?}", &data.lock().unwrap().active_chain);
                });
            });
        }
    });

    siv.menubar()
        .add_subtree(
            "File",
            menu::MenuTree::new()
                .leaf("Quit", |s| s.quit())
                .leaf("Help", |_| {}),
        )
        .add_subtree(
            "View",
            menu::MenuTree::new()
                .leaf("All offers", |_| {})
                .leaf("My offers", |_| {})
                .leaf("Currency offers", |_| {})
                .leaf("Identity offers", |_| {}),
        )
        .add_subtree("Select", menutree)
        .add_subtree(
            "New",
            menu::MenuTree::new()
                .leaf("Offer", |_| {})
                .leaf("Identity", |_| {})
                .leaf("Donation", |_| {}),
        )
        .add_delimiter()
        .add_leaf(format!("Search {}", '\u{1F50D}'), |s| {
            s.add_layer(Dialog::info("Search here"))
        });
    siv.set_autohide_menu(false);
}