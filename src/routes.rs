use dioxus::prelude::*;

use crate::views::home::Home;
use crate::views::menu::Menu;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/menu")]
    Menu {},
}
