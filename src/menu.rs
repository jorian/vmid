use crate::UserData;
use cursive::menu::MenuTree;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::Cursive;
use std::sync::Arc;
use tracing::debug;

pub fn new(siv: &mut Cursive, data: UserData) {
    let data_clone = Arc::clone(&data);
    siv.menubar()
        .add_subtree(
            "File",
            MenuTree::new()
                .leaf("Quit", |s| s.quit())
                .leaf("Help", |_| {}),
        )
        .add_subtree(
            "View",
            MenuTree::new()
                .leaf("All offers", |_| {})
                .leaf("My offers", |_| {})
                .leaf("Currency offers", |_| {})
                .leaf("Identity offers", |_| {}),
        )
        .add_subtree("Select", select_tree(Arc::clone(&data_clone)))
        .add_subtree(
            "New",
            MenuTree::new()
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

fn select_tree(data_clone: UserData) -> MenuTree {
    MenuTree::new().with(move |tree| {
        for chain in data_clone.lock().unwrap().local_chains.iter() {
            let chain = Arc::clone(chain);
            let name = chain.name.clone();

            tree.add_leaf(name, move |s| {
                s.with_user_data(|data: &mut UserData| {
                    data.lock().unwrap().active_chain = chain.clone();
                    debug!("new active_chain: {:?}", &data.lock().unwrap().active_chain);
                });
            });
        }
    })
}
