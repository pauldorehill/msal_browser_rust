//! App flow:
//! Build Client App -> sso login ->
//! interactive login -> acquire access token silent -> acquire access token interactive
//! logout

// Define all the js types in pure rust so that intellisense plays nice and can
// create a builder pattern

// TODO: Since working with WASM, use String over &str?
// TODO: Maybe split to features: Popup and Redirect? Since should use one or the other
// TODO: Build script that runs tests, and copies current msal-browser files to root?
// TODO: Test for each type going WASM -> JS -> WASM; then JSON?
mod msal;

#[cfg(feature = "popup")]
pub mod popup_app;
#[cfg(feature = "redirect")]
pub mod redirect_app;
pub mod requests;

use js_sys::{Array, Date};
use msal::{JsArrayString, JsMirror};
use requests::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

pub struct BrowserAuthOptions {
    client_id: String,
    authority: Option<String>,
    redirect_uri: Option<String>,
}

impl JsMirror for BrowserAuthOptions {
    type JsTarget = msal::BrowserAuthOptions;
}

impl From<BrowserAuthOptions> for msal::BrowserAuthOptions {
    fn from(auth_options: BrowserAuthOptions) -> Self {
        let auth = msal::BrowserAuthOptions::new(&auth_options.client_id);
        auth_options.authority.iter().for_each(|a| {
            auth.set_authority(&a);
        });
        auth_options.redirect_uri.iter().for_each(|ru| {
            auth.set_redirect_uri(ru);
        });
        auth
    }
}

impl BrowserAuthOptions {
    // Small strings so don't worry about 'leaked' memory on replace
    fn ref_set_authority(&mut self, authority: &str) {
        match self.authority.as_mut() {
            Some(s) => s.replace_range(.., authority),
            None => self.authority = Some(authority.to_string()),
        }
    }

    pub fn set_authority(mut self, authority: &str) -> Self {
        self.ref_set_authority(authority);
        self
    }

    fn ref_set_redirect_uri(&mut self, redirect_uri: &str) {
        match self.redirect_uri.as_mut() {
            Some(s) => s.replace_range(.., redirect_uri),
            None => self.redirect_uri = Some(redirect_uri.to_string()),
        }
    }

    pub fn set_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.ref_set_redirect_uri(redirect_uri);
        self
    }
}

impl From<&str> for BrowserAuthOptions {
    fn from(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            authority: None,
            redirect_uri: None,
        }
    }
}

pub struct Configuration {
    auth: BrowserAuthOptions,
}

impl JsMirror for Configuration {
    type JsTarget = msal::Configuration;
}

impl From<Configuration> for msal::Configuration {
    fn from(config: Configuration) -> Self {
        msal::Configuration::new(&config.auth.into())
    }
}

impl Configuration {
    pub fn set_authority(mut self, authority: &str) -> Self {
        self.auth.ref_set_authority(authority);
        self
    }

    pub fn set_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.auth.ref_set_redirect_uri(redirect_uri);
        self
    }
}

impl From<BrowserAuthOptions> for Configuration {
    fn from(browser_auth_options: BrowserAuthOptions) -> Self {
        Self {
            auth: browser_auth_options,
        }
    }
}

impl From<&str> for Configuration {
    fn from(client_id: &str) -> Self {
        let b: BrowserAuthOptions = client_id.into();
        b.into()
    }
}

// TODO: Date + work out what is going wrong passing token claims
// Check these are in UTC and do something better? Could just keep as Js Date and provide nicer methods with intellisense?
//file://./../node_modules/@azure/msal-common/dist/src/response/AuthenticationResult.d.ts
pub struct AuthenticationResult {
    pub unique_id: String,
    pub tenant_id: String,
    pub scopes: Vec<String>,
    pub account: AccountInfo,
    pub id_token: String,
    // pub id_token_claims: HashMap<String, String>,
    pub access_token: String,
    pub from_cache: bool,
    pub expires_on: Date,
    pub ext_expires_on: Option<Date>,
    pub state: Option<String>,
    pub family_id: Option<String>,
}

impl From<msal::AuthenticationResult> for AuthenticationResult {
    fn from(auth_result: msal::AuthenticationResult) -> Self {
        Self {
            unique_id: auth_result.unique_id(),
            tenant_id: auth_result.tenant_id(),
            scopes: JsArrayString::from(auth_result.scopes()).0,
            account: auth_result.account().into(),
            id_token: auth_result.id_token(),
            // id_token_claims: JsHashMapStringString::from(auth_result.id_token_claims()).0,
            access_token: auth_result.access_token(),
            from_cache: auth_result.from_cache(),
            expires_on: auth_result.expires_on(),
            ext_expires_on: auth_result.ext_expires_on(),
            state: auth_result.state(),
            family_id: auth_result.family_id(),
        }
    }
}

impl From<JsValue> for AuthenticationResult {
    fn from(value: JsValue) -> Self {
        value.unchecked_into::<msal::AuthenticationResult>().into()
    }
}

/// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/initialization.md#choosing-an-interaction-type
/// there are two apis: popup and redirect. Redirect requires that you must call `handleRedirectPromise()`
/// because of this I will split into two and also crate a trait

pub trait PublicClientApplication {
    fn auth(&self) -> &msal::PublicClientApplication;

    fn empty_request() -> msal::AuthorizationUrlRequest {
        msal::AuthorizationUrlRequest::new(&Array::new())
    }

    fn client_id(&self) -> String {
        self.auth().config().auth().client_id()
    }

    fn authority(&self) -> String {
        self.auth().config().auth().authority()
    }

    fn redirect_uri(&self) -> String {
        self.auth().config().auth().redirect_uri()
    }

    fn get_all_accounts(&self) -> Vec<AccountInfo> {
        AccountInfo::from_array(self.auth().get_all_accounts())
    }

    fn get_account_by_username(&self, username: &str) -> AccountInfo {
        self.auth()
            .get_account_by_username(username.to_string())
            .into()
    }
    fn logout(&self, request: Option<EndSessionRequest>) {
        self.auth().logout(request.unwrap_or_default().into())
    }
}

// Can't put these on the trait since `async` is not allowed in traits
// https://rust-lang.github.io/async-book/07_workarounds/06_async_in_traits.html
// https://github.com/dtolnay/async-trait

/// Silent login https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/login-user.md#silent-login-with-ssosilent
/// Can use this first to try loging in without interation, if it fails will then need to run the normal login work flow
/// Note the 'account' (i think authority) must be set for this to run
async fn sso_silent(
    client_app: &msal::PublicClientApplication,
    request: AuthorizationUrlRequest,
) -> Result<AuthenticationResult, JsValue> {
    client_app.sso_silent(request.into()).await.map(Into::into)
}

/// Called by both popup and redirect
/// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/acquire-token.md
/// Call this first, then if it fails will will need to call the interative methods
async fn acquire_token_silent(
    client_app: &msal::PublicClientApplication,
    request: SilentRequest,
) -> Result<AuthenticationResult, JsValue> {
    client_app
        .acquire_token_silent(request.into())
        .await
        .map(Into::into)
}

pub struct AccountInfo {
    pub home_account_id: String,
    pub environment: String,
    pub tenant_id: String,
    pub username: String,
}

impl JsMirror for AccountInfo {
    type JsTarget = msal::AccountInfo;
}

impl From<msal::AccountInfo> for AccountInfo {
    fn from(account_info: msal::AccountInfo) -> Self {
        Self {
            home_account_id: account_info.home_account_id(),
            environment: account_info.environment(),
            tenant_id: account_info.tenant_id(),
            username: account_info.username(),
        }
    }
}

impl From<AccountInfo> for msal::AccountInfo {
    fn from(account_info: AccountInfo) -> Self {
        msal::AccountInfo::new(
            account_info.home_account_id,
            account_info.environment,
            account_info.tenant_id,
            account_info.username,
        )
    }
}

impl AccountInfo {
    pub fn from_array(array: Array) -> Vec<Self> {
        array
            .iter()
            .map(|v| v.unchecked_into::<msal::AccountInfo>().into())
            .collect()
    }
}

pub mod prelude {
    pub use crate::popup_app::PopupApp;
    pub use crate::requests::*;
    pub use crate::{
        AccountInfo, AuthenticationResult, BrowserAuthOptions, Configuration,
        PublicClientApplication,
    };
}

#[cfg(test)]
mod tests_in_browser {
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::*;
    use wasm_bindgen_test::*;

    fn home_account_id(i: usize) -> String {
        format!("home_account_id_{}", i)
    }
    fn environment(i: usize) -> String {
        format!("environment_{}", i)
    }
    fn tenant_id(i: usize) -> String {
        format!("tenant_id_{}", i)
    }
    fn username(i: usize) -> String {
        format!("username_{}", i)
    }

    // Make on the Js side
    fn make_account_info(i: usize) -> msal::AccountInfo {
        msal::AccountInfo::new(
            home_account_id(i),
            environment(i),
            tenant_id(i),
            username(i),
        )
    }

    #[wasm_bindgen_test]
    fn convert_account_info() {
        let len: usize = 10;
        let xs = Array::new();
        for i in 0..len {
            xs.push(&make_account_info(i));
        }

        let accounts = AccountInfo::from_array(xs);

        for i in 0..len {
            assert_eq!(home_account_id(i), accounts[i].home_account_id);
            assert_eq!(environment(i), accounts[i].environment);
            assert_eq!(tenant_id(i), accounts[i].tenant_id);
            assert_eq!(username(i), accounts[i].username);
        }
    }

    #[wasm_bindgen_test]
    fn convert_authentication_result() {
        let unique_id = "unique_id".to_string();
        let tenant_id = "tenant_id".to_string();
        let scopes = JsArrayString::from("scopes");
        let account = make_account_info(0);
        let id_token = "id_token".to_string();
        // let id_token_claims = JsHashMapStringString::from(("claim key", "claim value"));
        let access_token = "access token".to_string();
        let from_cache = false;
        let expires_on = Date::new_0();
        let state = Some("state".to_string());
        let family_id = Some("family_id".to_string());

        let ar = msal::AuthenticationResult::new();
        ar.set_unique_id(unique_id.clone());
        ar.set_tenant_id(tenant_id.clone());
        ar.set_scopes(scopes.clone().into());
        ar.set_account(account.clone().into());
        ar.set_id_token(id_token.clone());
        // ar.set_id_token_claims(id_token_claims.clone().into());
        ar.set_access_token(access_token.clone());
        ar.set_from_cache(from_cache);
        ar.set_expires_on(expires_on.clone());
        ar.set_ext_expires_on(Some(expires_on.clone()));
        ar.set_state(state.clone());
        ar.set_family_id(family_id.clone());

        let ar_wasm: AuthenticationResult = ar.into();
        assert_eq!(ar_wasm.unique_id, unique_id);
        assert_eq!(ar_wasm.tenant_id, tenant_id);
        assert_eq!(ar_wasm.scopes, scopes.0);
        assert_eq!(ar_wasm.account.environment, account.environment());
        assert_eq!(ar_wasm.account.home_account_id, account.home_account_id());
        assert_eq!(ar_wasm.account.tenant_id, account.tenant_id());
        assert_eq!(ar_wasm.account.username, account.username());
        assert_eq!(ar_wasm.id_token, id_token);
        // assert_eq!(ar_wasm.id_token_claims, id_token_claims.0);
        assert_eq!(ar_wasm.access_token, access_token);
        assert_eq!(ar_wasm.from_cache, from_cache);
        assert_eq!(ar_wasm.expires_on, expires_on);
        assert_eq!(ar_wasm.ext_expires_on, Some(expires_on));
        assert_eq!(ar_wasm.state, state);
        assert_eq!(ar_wasm.family_id, family_id);
    }
}
