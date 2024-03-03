use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;

mod current_user;
mod home;
mod login;

pub fn routes() -> Router<AppState> {
    Router::new()
        // .route("/styles/tailwind.css", get(tailwind_css))
        .route("/login", get(login::show))
        .route("/login/callback", get(login::callback))
        .route("/logout", post(login::logout))
        // .route("/", get(home::show))
        // .route(
        //     "/my/sites",
        //     get(current_user::sites::index).post(current_user::sites::create),
        // )
        .route("/my/sites/new", get(current_user::sites::new))
        .route("/my/sites/:site_id", get(current_user::sites::show))
        .route(
            "/my/sites/:site_id/pages/new",
            get(current_user::pages::new),
        )
        .route(
            "/my/sites/:site_id/pages",
            post(current_user::pages::create),
        )
        .route(
            "/my/sites/:site_id/pages/:page_id",
            get(current_user::pages::show),
        )
}

// async fn tailwind_css() -> &'static str {
//     include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/tailwind.css"))
// }
