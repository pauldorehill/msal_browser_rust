# Rust + WASM + msal-browser
Rust wrapper for [msal-browser.js](https://github.com/AzureAD/microsoft-authentication-library-for-js). Still under dev, but you can login, get tokens and logout.

Approx file sizes in kb:

| File | Debug | Release | Release + minified
| --- | --- | --- | --- |
| wasm | 63 | 14 | 14
| js | 363 | 363 | 113

The js file bundles the `msal-browser.js` library.

Methods names all match the `js` but with snake case, but unlike the `js` instead of a single `PublicClientApplication` type there are two app types:

- `PopupApp`
- `RedirectApp`

The `PopupApp` is a the default feature: if you want to use the `RedirectApp` it is behind the `redirect` feature:

```rust
msal_browser = { version = "0.1", features = ["redirect"] }
```

There are a huge amount of [`Configuration`](https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md) options so the rust side uses a builder pattern. You can also use a js `Object` and call `Configuration::TryFrom<Object>`.

To use:


```rust
const CLIENT_ID: &str = "YOUR_CLIENT_ID";
const AUTHORITY: &str = "YOUR_AUTHORITY";

// Setup App and build
let auth_options = BrowserAuthOptions::from(CLIENT_ID).set_authority(AUTHORITY);
let config = Configuration::from(auth_options);
let client_app = PopupApp::new(config);

let scopes = ["User.Read"];

// Login
let auth_res = client_app.login_popup().await.unwrap();

// Get account
let account = &client_app.get_all_accounts().unwrap()[0];

// Setup some requests
let base_request = BaseAuthRequest::from(&scopes[..]);
let auth_request =
    AuthorizationUrlRequest::from(&base_request).set_login_hint(account.username());
let silent_request = SilentRequest::from_account_info(&base_request, account);

// SSO sign in
let sso_auth_result = client_app.sso_silent(&auth_request).await.unwrap();

// Popup token
let token = client_app.acquire_token_popup(&auth_request).await.unwrap();

// Silent token
let silent_token = client_app.acquire_token_silent(&silent_request).await.unwrap();

// Logout
client_app.logout(None);
```