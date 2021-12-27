use crate::data::Data;
use cursive::menu;
use cursive::traits::*;
use cursive::views::Dialog;
use cursive::Cursive;
use tracing::info;

use std::{cell::RefCell, rc::Rc};

pub fn new(siv: &mut Cursive, data: Rc<RefCell<Data>>) {
    let data_clone = Rc::clone(&data);

    let menutree = cursive::menu::MenuTree::new().with(move |tree| {
        for chain in data_clone.borrow().local_chains.iter() {
            let chain = Rc::clone(chain);
            let name = chain.borrow().name.clone();

            tree.add_leaf(name, move |s| {
                s.with_user_data(|data: &mut Rc<RefCell<Data>>| {
                    data.borrow_mut().active_chain = chain.clone();
                    info!("{:?}", &data.borrow().active_chain);
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
