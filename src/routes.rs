use dioxus::prelude::*;

use crate::views::home::Home;
use crate::views::menu::Menu;
use crate::views::login::Login;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/menu")]
    Menu {},

    #[route("/login")]
    Login {},
}
