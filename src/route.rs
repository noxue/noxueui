use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}
