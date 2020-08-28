use dominator::{clone, events, html, Dom};
use futures_signals::signal::{Mutable, Signal};
use msal_browser::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

#[derive(Clone)]
pub struct Auth {
    popup_app: PopupApp,
    user: Mutable<Option<AccountInfo>>,
}

impl Auth {
    pub fn new() -> Self {
        // Set your own client Id & authority here however you like.
        // It is done like this so I can exlude the details from source control for my testing
        // Typically something like:
        // clientId: "3fba556e-5d4a-48e3-8e1a-fd57c12cb82e",
        // authority: "https://login.windows-ppe.net/common/"
        let credentials: Vec<&str> = include_str!("../credentials.txt").split("::").collect();
        let client_id = credentials[0];
        let authority = credentials[1];

        let auth_options = BrowserAuthOptions::new(client_id).set_authority(authority);
        let config = Configuration::new(auth_options);
        let auth = PopupApp::new(config);

        match auth.get_all_accounts() {
            None => Self {
                popup_app: auth,
                user: Mutable::new(None),
            },
            Some(accounts) if accounts.len() == 1 => Self {
                popup_app: auth,
                user: Mutable::new(Some(accounts[0].clone())),
            },
            // TODO: Handle multiple accounts
            Some(_accounts) => Self {
                popup_app: auth,
                user: Mutable::new(None),
            },
        }
    }

    pub fn render_login(auth: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        auth.user.clone().signal_ref(move |account| {
            Some(if account.is_some() {
                html!("div", {
                    .children(&mut [
                        html!("div", {
                            .text("Username: ")
                            .text(account.as_ref().unwrap().username())
                        }),
                        html!("div", {
                            .text("Tenant Id: ")
                            .text(account.as_ref().unwrap().tenant_id())
                        }),
                        html!("div", {
                            .text("Home Account Id: ")
                            .text(account.as_ref().unwrap().home_account_id())
                        }),
                        html!("div", {
                            .text("Environment: ")
                            .text(account.as_ref().unwrap().environment())
                        }),
                        html!("button", {
                            .text("Logout")
                            .event(clone!(auth => move |_: events::Click| {
                                auth.popup_app.logout(None);
                                auth.user.set(None);
                            }))
                        })
                    ])
                })
            } else {
                let auth = Rc::clone(&auth);
                let event = move || {
                    let auth = Rc::clone(&auth);
                    spawn_local(async move {
                        match auth.popup_app.login_popup().await {
                            Ok(ar) => auth.user.set(Some(ar.account().clone())),
                            Err(js) => {
                                console::log_1(&"Login failed:".into());
                                console::log_1(&js)
                            }
                        }
                    })
                };
                html!("button", {
                    .text("Login")
                    .event(move |_: events::Click| event())
                })
            })
        })
    }

    pub fn render(auth: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(Self::render_login(auth))
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let auth = Rc::new(Auth::new());
    dominator::append_dom(&dominator::body(), Auth::render(auth));
    Ok(())
}

const CLIENT_ID: &str = "CLIENT_ID";
const AUTHORITY: &str = "AUTHORITY";

/// Example Api
#[allow(unused_variables)]
#[allow(dead_code)]
async fn popup_example() {
    // Setup App
    let auth_options = BrowserAuthOptions::new(CLIENT_ID).set_authority(AUTHORITY);
    let config = Configuration::new(auth_options);
    let client_app = PopupApp::new(config);

    // Define some scopes
    let scopes = ["User.Read"];

    // Login
    let auth_res = client_app.login_popup().await.unwrap();
    let auth_res = client_app.login_popup_with_scopes(&scopes).await.unwrap();

    // Account Info
    let account = client_app.get_account_by_username("username").unwrap();
    let account = client_app.get_account_by_home_id("home_id").unwrap();
    let accounts = &client_app.get_all_accounts();

    // Requests
    let auth_request = AuthorizationUrlRequest::new(&scopes[..]).set_login_hint(account.username());
    let silent_request = SilentRequest::new(&scopes[..], &account);
    let end_session_request = EndSessionRequest::new();

    // SSO sign in
    let sso_auth_result = client_app.sso_silent(&auth_request).await.unwrap();

    // Popup token
    let token = client_app.acquire_token_popup(&auth_request).await.unwrap();

    // Silent token
    let silent_token = client_app
        .acquire_token_silent(&silent_request)
        .await
        .unwrap();

    // Logout
    client_app.logout(None);
}
