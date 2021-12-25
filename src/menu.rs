use cursive::menu;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::Cursive;
use tracing::{error, info};

use crate::data::Chain;
use crate::util;

pub fn new(siv: &mut Cursive) {
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
        .add_subtree("Select", menu::MenuTree::new())
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

// fn set_active_chain(s: &mut Cursive, chain: Chain) {
//     info!("{} selected", &chain);
//     s.call_on_name("chain_name", |view: &mut cursive::views::TextView| {
//         view.set_content(&chain.to_string())
//     });
//     if let None = s.with_user_data(|data: &mut crate::data::Data| {
//         data.active_chain.chain = chain.clone();
//         data.local_chains.push(chain)
//     }) {
//         error!("user data was not updated, undefined behaviour");
//         panic!("user_data not set");
//     }
// }
