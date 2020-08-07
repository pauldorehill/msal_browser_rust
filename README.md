# Rust + WASM + msal-browser
Rust wrapper for [msal-browser.js](https://github.com/AzureAD/microsoft-authentication-library-for-js). Still under dev, but you can login, get tokens and logout.

Methods names all match the `js` but with snake case, but unlike the `js` that exposes all methods on a `PublicClientApplication` type there are two app types:

- `PopupApp`
- `RedirectApp`

The `PopupApp` is a the default feature: if you want to use the `RedirectApp` it is behind the `redirect` feature:

```rust
msal_browser = { version = "0.1", features = ["redirect"] }
```

There are a huge amount of [`Configuration`](https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md) options so the rust side uses a builder pattern. 

(You can also use a js `Object` and call `Configuration::TryFrom<Object> / Into`... **however note that if the fields aren't yet on the builder they won't get added**)

To use:


```rust
const CLIENT_ID: &str = "YOUR_CLIENT_ID";
const AUTHORITY: &str = "YOUR_AUTHORITY";

// Setup App and build
let auth_options = BrowserAuthOptions::from(CLIENT_ID).set_authority(AUTHORITY);
let config = Configuration::from(auth_options);
let client_app = PopupApp::new(config);

// Login
let auth_res = client_app.login_popup().await.unwrap();

```