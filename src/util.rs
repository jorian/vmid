use super::data::*;

// A non-failing function that finds all the installed chains: VRSC, VRSCTEST and the used PBaaS installations
// (.komodo/VRSC, .komodo/VRSCTEST, .verustest/pbaas/* )
pub(crate) fn find_local_chain_installations() -> Vec<Chain> {
    // TODO
    vec![
        Chain(String::from("VRSC")),
        Chain(String::from("Mutt")),
        Chain(String::from("VRSCTEST")),
    ]
}
