# Rust + WASM + msal-browser
Rust wrapper for [msal-browser.js](https://github.com/AzureAD/microsoft-authentication-library-for-js).

Methods names all match the `js` but with snake case, but unlike the `js` instead of a single `PublicClientApplication` type there are two app types:

- `PopupApp`
- `RedirectApp`

The `PopupApp` is a the default feature: if you want to use the `RedirectApp` it is behind the `redirect` feature:

```rust
msal_browser = { version = "0.2.0", features = ["redirect"] }
```

There are a huge amount of [`Configuration`](https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md) options so the rust side uses a builder pattern. You can also use a js `Object` and call `Configuration::unchecked_from`.

To use:


```rust
const CLIENT_ID: &str = "YOUR_CLIENT_ID";
const AUTHORITY: &str = "YOUR_AUTHORITY";

// Setup App
let auth_options = BrowserAuthOptions::new(CLIENT_ID).set_authority(AUTHORITY);
let config = Configuration::new(auth_options);
let client_app = PopupApp::new(config);

// Define some scopes
let scopes = ["User.Read"];

// Login
let auth_res = client_app.login_popup().await.unwrap();

// Get account
let account = &client_app.get_all_accounts().unwrap()[0];

// Setup some requests
let auth_request = AuthorizationUrlRequest::new(&scopes[..]).set_login_hint(account.username());
let silent_request = SilentRequest::new(&scopes[..], account);

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
```

Approx file sizes in kb:

| File | Debug | Release | Release + minified
| --- | --- | --- | --- |
| wasm | 63 | 14 | 14
| js | 363 | 363 | 113

The js file bundles the `msal-browser.js` library.