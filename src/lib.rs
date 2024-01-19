use cfg_if::cfg_if;
pub mod app;
pub mod app_context;
pub mod error_template;
pub mod fileserv;
pub mod password;
pub mod password_form;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Info);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}
