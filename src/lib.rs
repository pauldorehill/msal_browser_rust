//! msal-browser.js in Rust WASM

mod msal;
// TODO: Since working with WASM, I used String - use &str?
// TODO: doc build for all features

#[cfg(feature = "popup")]
pub mod popup_app;
#[cfg(feature = "redirect")]
pub mod redirect_app;
pub mod requests;

use js_sys::{Array, Date, Object};
use msal::{JsArrayString, JsMirror};
use requests::*;
use std::{collections::HashMap, convert::TryFrom};
use wasm_bindgen::{JsCast, JsValue};

// TODO: Is this worth it? May 'leak' but avoids allocation
fn set_option_string(current_value: &mut Option<String>, new_value: &str) {
    match current_value {
        None => *current_value = Some(String::from(new_value)),
        Some(s) => s.replace_range(.., new_value),
    }
}

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
        auth_options.redirect_uri.iter().for_each(|uri| {
            auth.set_redirect_uri(uri);
        });
        auth
    }
}

impl From<msal::BrowserAuthOptions> for BrowserAuthOptions {
    fn from(auth: msal::BrowserAuthOptions) -> Self {
        Self {
            client_id: auth.client_id(),
            authority: auth.authority(),
            redirect_uri: auth.redirect_uri(),
        }
    }
}

impl BrowserAuthOptions {
    // Small strings so don't worry about 'leaked' memory on replace
    fn ref_set_authority(&mut self, authority: &str) {
        set_option_string(&mut self.authority, authority)
    }

    pub fn set_authority(mut self, authority: &str) -> Self {
        self.ref_set_authority(authority);
        self
    }

    fn ref_set_redirect_uri(&mut self, redirect_uri: &str) {
        set_option_string(&mut self.redirect_uri, redirect_uri)
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

// TODO: Add in all fields
pub struct Configuration {
    auth: BrowserAuthOptions,
    // cache: Option<CacheOptions>,
    // system: Option<BrowserSystemOptions>,
}

impl JsMirror for Configuration {
    type JsTarget = msal::Configuration;
}

impl From<Configuration> for msal::Configuration {
    fn from(config: Configuration) -> Self {
        msal::Configuration::new(&config.auth.into())
    }
}

impl From<msal::Configuration> for Configuration {
    fn from(config: msal::Configuration) -> Self {
        Self {
            auth: config.auth().into(),
        }
    }
}

impl TryFrom<Object> for Configuration {
    type Error = JsValue;
    fn try_from(js_obj: Object) -> Result<Self, Self::Error> {
        let v: Configuration = js_obj.unchecked_into::<msal::Configuration>().into();
        Ok(v)
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

// TODO: Should this move to an enum of all claims? Access & Id, how to handle optional / custom?
// https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-optional-claims#configuring-directory-extension-optional-claims
// https://tools.ietf.org/html/rfc7519#section-4.1
// https://tools.ietf.org/html/rfc7515
// https://www.iana.org/assignments/jwt/jwt.xhtml#claims
#[derive(Clone, PartialEq, Debug)]
pub enum TokenValue {
    String(String),
    Float(f64),
}

// TODO: Date is a Js type, should I change?
//file://./../node_modules/@azure/msal-common/dist/src/response/AuthenticationResult.d.ts
pub struct AuthenticationResult {
    pub unique_id: String,
    pub tenant_id: String,
    pub scopes: Vec<String>,
    pub account: AccountInfo,
    pub id_token: String,
    pub id_token_claims: HashMap<String, TokenValue>,
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
            id_token_claims: msal::TokenHashMap::from(auth_result.id_token_claims()).0,
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

pub trait PublicClientApplication {
    fn auth(&self) -> &msal::PublicClientApplication;

    fn empty_request() -> msal::AuthorizationUrlRequest {
        msal::AuthorizationUrlRequest::new(&Array::new())
    }

    fn client_id(&self) -> String {
        self.auth().config().auth().client_id()
    }

    fn authority(&self) -> Option<String> {
        self.auth().config().auth().authority()
    }

    fn redirect_uri(&self) -> Option<String> {
        self.auth().config().auth().redirect_uri()
    }

    fn get_all_accounts(&self) -> Option<Vec<AccountInfo>> {
        self.auth().get_all_accounts().map(AccountInfo::from_array)
    }

    fn get_account_by_username(&self, username: &str) -> Option<AccountInfo> {
        self.auth()
            .get_account_by_username(username.to_string())
            .map(Into::into)
    }

    fn logout(&self, request: Option<EndSessionRequest>) {
        self.auth().logout(request.unwrap_or_default().into())
    }
}

// Can't put these on the trait since `async` is not allowed in traits
// https://rust-lang.github.io/async-book/07_workarounds/06_async_in_traits.html
// https://github.com/dtolnay/async-trait

// Silent login https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/login-user.md#silent-login-with-ssosilent
// needs a login_hint, sid or account object on the request
async fn sso_silent(
    client_app: &msal::PublicClientApplication,
    request: AuthorizationUrlRequest,
) -> Result<AuthenticationResult, JsValue> {
    client_app.sso_silent(request.into()).await.map(Into::into)
}

// Called by both popup and redirect
// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/acquire-token.md
// Call this first, then if it fails will will need to call the interactive methods
async fn acquire_token_silent(
    client_app: &msal::PublicClientApplication,
    request: SilentRequest,
) -> Result<AuthenticationResult, JsValue> {
    client_app
        .acquire_token_silent(request.into())
        .await
        .map(Into::into)
}

#[derive(Clone)]
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
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::*;
    use js_sys::Object;
    use std::convert::TryInto;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;

    pub const HOME_ACCOUNT_ID: &str = "home_account_id";
    pub const ENVIRONMENT: &str = "environment";
    pub const TENANT_ID: &str = "tenant_id";
    pub const USERNAME: &str = "username";
    pub const SCOPE: &str = "scope";
    pub const AUTHORITY: &str = "authority";
    pub const CORRELATION_ID: &str = "correlation_id";
    pub const POST_LOGOUT_URI: &str = "correlation_id";

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

    pub fn account() -> AccountInfo {
        AccountInfo {
            home_account_id: HOME_ACCOUNT_ID.to_string(),
            environment: ENVIRONMENT.to_string(),
            tenant_id: TENANT_ID.to_string(),
            username: USERNAME.to_string(),
        }
    }

    pub fn js_cast_checker<T>(js: JsValue)
    where
        T: JsCast,
    {
        match js.dyn_into::<T>() {
            Ok(_) => (),
            Err(_) => panic!("failed js cast"),
        }
    }

    // Make on the Js side
    fn make_account_info_in_js_land(i: usize) -> msal::AccountInfo {
        msal::AccountInfo::new(
            home_account_id(i),
            environment(i),
            tenant_id(i),
            username(i),
        )
    }

    #[wasm_bindgen_test]
    fn mirror_account_info() {
        let account = account();
        let js_ac: msal::AccountInfo = account.clone().into();
        assert_eq!(js_ac.home_account_id(), account.home_account_id);
        assert_eq!(js_ac.environment(), account.environment);
        assert_eq!(js_ac.tenant_id(), account.tenant_id);
        assert_eq!(js_ac.username(), account.username);
    }

    #[wasm_bindgen_test]
    fn convert_account_info_array() {
        let len: usize = 10;
        let js_accounts = Array::new();
        for i in 0..len {
            js_accounts.push(&make_account_info_in_js_land(i));
        }

        let accounts = AccountInfo::from_array(js_accounts);

        for (i, account) in accounts.iter().enumerate() {
            assert_eq!(home_account_id(i), account.home_account_id);
            assert_eq!(environment(i), account.environment);
            assert_eq!(tenant_id(i), account.tenant_id);
            assert_eq!(username(i), account.username);
        }
    }

    #[wasm_bindgen(module = "/msal-object-examples.js")]
    extern "C" {
        static authResponse: Object;
        static msalConfig: Object;
    }

    #[wasm_bindgen_test]
    fn parse_authentication_result() {
        let _: AuthenticationResult = authResponse
            .clone()
            .unchecked_into::<msal::AuthenticationResult>()
            .into();
    }

    #[wasm_bindgen_test]
    fn parse_config_result() {
        let _: Configuration = msalConfig.clone().try_into().unwrap();
    }

    #[wasm_bindgen_test]
    fn mirror_configuration() {
        // TODO: Write tests
    }

    #[wasm_bindgen_test]
    fn mirror_browser_auth_options() {
        // TODO: Write tests
    }
}
