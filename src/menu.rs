use cursive::menu::MenuTree;
use cursive::views::Dialog;
use cursive::Cursive;

use crate::{Chain, Orderbook};

pub fn new(siv: &mut Cursive) {
    siv.menubar()
        .add_subtree("File", MenuTree::new().leaf("Quit", |s| s.quit()))
        .add_subtree(
            "View",
            MenuTree::new()
                .leaf("All offers", |_| {})
                .leaf("My offers", |_| {})
                .leaf("Currency offers", |_| {})
                .leaf("Identity offers", |_| {}),
        )
        .add_subtree(
            "Select",
            MenuTree::new()
                .leaf("VRSC", |s| {
                    s.with_user_data(|ob: &mut Orderbook| {
                        dbg!(&ob.chain);
                        ob.chain = Chain::VRSC
                    });
                })
                .leaf("VRSCTEST", |s| {
                    s.with_user_data(|ob: &mut Orderbook| {
                        dbg!(&ob.chain);
                        ob.chain = Chain::VRSCTEST
                    });
                })
                .leaf("mutt", |s| {
                    s.with_user_data(|ob: &mut Orderbook| {
                        dbg!(&ob.chain);
                        ob.chain = Chain::Mutt
                    });
                }),
        )
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
        })
        .add_leaf("Help", |_| {});

    siv.set_autohide_menu(false);
}
