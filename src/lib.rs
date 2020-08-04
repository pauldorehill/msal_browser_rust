//! App flow:
//! Build Client App -> sso login ->
//! interactive login -> acquire access token silent -> acquire access token interactive
//! logout
//! Since working with WASM, use String over &str?

#![allow(dead_code)]
// When this is turned on all my intellisense dies ;-(
#![cfg(target_arch = "wasm32")]

mod msal;

use js_sys::{Array, Date};
use msal::*;
use std::{collections::HashMap, fmt::Display};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

struct BaseAuthRequest {
    scopes: Vec<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl From<Vec<String>> for BaseAuthRequest {
    fn from(scopes: Vec<String>) -> Self {
        Self {
            scopes,
            authority: None,
            correlation_id: None,
        }
    }
}

impl From<Vec<&str>> for BaseAuthRequest {
    fn from(scopes: Vec<&str>) -> Self {
        scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .into()
    }
}

enum ResponseMode {
    Query,
    Fragment,
    FormPost,
}

impl Display for ResponseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ResponseMode::Query => write!(f, "query"),
            ResponseMode::Fragment => write!(f, "fragment"),
            ResponseMode::FormPost => write!(f, "form_post"),
        }
    }
}

pub struct AuthorizationUrlRequest {
    base_request: BaseAuthRequest,
    redirect_uri: Option<String>,
    extra_scopes_to_consent: Option<Vec<String>>,
    response_mode: Option<ResponseMode>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
    state: Option<String>,
    prompt: Option<String>,
    login_hint: Option<String>,
    domain_hint: Option<String>,
    extra_query_parameters: Option<HashMap<String, String>>,
    claims: Option<String>,
    nonce: Option<String>,
}

impl AuthorizationUrlRequest {
    //TODO: Complete build steps
    fn build(&self) -> msal::AuthorizationUrlRequest {
        let auth_req = msal::AuthorizationUrlRequest::new(
            &JsArrayString::from(self.base_request.scopes.clone()).into(),
        );
        if let Some(authority) = &self.base_request.authority {
            auth_req.set_authority(authority.clone());
        }
        if let Some(correlation_id) = &self.base_request.correlation_id {
            auth_req.set_correlation_id(correlation_id.clone());
        }
        auth_req
    }
}

impl From<AuthorizationUrlRequest> for msal::AuthorizationUrlRequest {
    fn from(request: AuthorizationUrlRequest) -> Self {
        request.build()
    }
}

impl From<Vec<&str>> for AuthorizationUrlRequest {
    fn from(scopes: Vec<&str>) -> Self {
        scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .into()
    }
}

impl From<Vec<String>> for AuthorizationUrlRequest {
    fn from(scopes: Vec<String>) -> Self {
        Self {
            base_request: BaseAuthRequest::from(scopes),
            redirect_uri: None,
            extra_scopes_to_consent: None,
            response_mode: None,
            code_challenge: None,
            code_challenge_method: None,
            state: None,
            prompt: None,
            login_hint: None,
            domain_hint: None,
            extra_query_parameters: None,
            claims: None,
            nonce: None,
        }
    }
}

impl From<BaseAuthRequest> for AuthorizationUrlRequest {
    fn from(base_request: BaseAuthRequest) -> Self {
        Self {
            base_request,
            redirect_uri: None,
            extra_scopes_to_consent: None,
            response_mode: None,
            code_challenge: None,
            code_challenge_method: None,
            state: None,
            prompt: None,
            login_hint: None,
            domain_hint: None,
            extra_query_parameters: None,
            claims: None,
            nonce: None,
        }
    }
}

pub struct RedirectRequest {
    auth_url_req: AuthorizationUrlRequest,
    redirect_start_page: Option<String>,
}

impl RedirectRequest {
    pub fn new(scopes: Vec<String>) -> Self {
        Self {
            auth_url_req: AuthorizationUrlRequest::from(scopes),
            redirect_start_page: None,
        }
    }
    // TODO: Add all values
    fn build(&self) -> msal::RedirectRequest {
        let auth_req = msal::RedirectRequest::new(
            &JsArrayString::from(self.auth_url_req.base_request.scopes.clone()).into(),
        );
        auth_req
    }
}

impl From<Vec<&str>> for RedirectRequest {
    fn from(scopes: Vec<&str>) -> Self {
        scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .into()
    }
}

impl From<Vec<String>> for RedirectRequest {
    fn from(scopes: Vec<String>) -> Self {
        Self {
            auth_url_req: scopes.into(),
            redirect_start_page: None,
        }
    }
}

impl From<RedirectRequest> for msal::RedirectRequest {
    fn from(request: RedirectRequest) -> Self {
        request.build()
    }
}

// Adds acccount and force refresh to BaseAuthRequest
pub struct SilentRequest {
    base_request: BaseAuthRequest,
    account: AccountInfo,
    force_refresh: Option<bool>,
    redirect_uri: Option<String>,
}

#[derive(Default)]
pub struct EndSessionRequest {
    account: Option<String>,
    post_logout_redirect_uri: Option<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl From<EndSessionRequest> for msal::EndSessionRequest {
    fn from(request: EndSessionRequest) -> Self {
        let r = msal::EndSessionRequest::new();
        if let Some(account) = request.account {
            r.set_account(account);
        }
        if let Some(post_logout_redirect_uri) = request.post_logout_redirect_uri {
            r.set_account(post_logout_redirect_uri);
        }
        if let Some(authority) = request.authority {
            r.set_account(authority);
        }
        if let Some(correlation_id) = request.correlation_id {
            r.set_account(correlation_id);
        }
        r
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
    pub(crate) expires_on: Date,
    pub(crate) ext_expires_on: Option<Date>,
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
// Define these in pure rust so that intellisense plays nice and can
// create a builder pattern

pub struct BrowserAuthOptions {
    client_id: String,
    authority: Option<String>,
    redirect_uri: Option<String>,
}

impl BrowserAuthOptions {
    pub fn new(client_id: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            authority: None,
            redirect_uri: None,
        }
    }

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

    fn build(&self) -> msal::BrowserAuthOptions {
        let auth = msal::BrowserAuthOptions::new(&self.client_id);
        self.authority.iter().for_each(|a| {
            auth.set_authority(&a);
        });
        self.redirect_uri.iter().for_each(|ru| {
            auth.set_redirect_uri(ru);
        });
        auth
    }
}

impl From<&str> for BrowserAuthOptions {
    fn from(client_id: &str) -> Self {
        Self::new(client_id)
    }
}

pub struct Configuration {
    auth: BrowserAuthOptions,
}

impl Configuration {
    pub fn new(browser_auth_options: BrowserAuthOptions) -> Self {
        Self {
            auth: browser_auth_options,
        }
    }

    pub fn set_authority(mut self, authority: &str) -> Self {
        self.auth.ref_set_authority(authority);
        self
    }

    pub fn set_redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.auth.ref_set_redirect_uri(redirect_uri);
        self
    }

    fn build(self) -> msal::Configuration {
        msal::Configuration::new(&self.auth.build())
    }
}

impl From<BrowserAuthOptions> for Configuration {
    fn from(browser_auth_options: BrowserAuthOptions) -> Self {
        Self::new(browser_auth_options)
    }
}

impl From<&str> for Configuration {
    fn from(client_id: &str) -> Self {
        Self::new(client_id.into())
    }
}

/// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/initialization.md#choosing-an-interaction-type
/// there are two apis: popup and redirect. Redirect requires that you must call `handleRedirectPromise()`
/// because of this I will split into two and also crate a trait

/// : private::Sealed? - doesn't compile
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

/// Silent login https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/login-user.md#silent-login-with-ssosilent
/// Can use this first to try loging in without interation, if it fails will then need
/// run the normal login work flow
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
    request: AuthorizationUrlRequest,
) -> Result<AuthenticationResult, JsValue> {
    client_app
        .acquire_token_silent(request.into())
        .await
        .map(Into::into)
}

pub struct RedirectApp<FSuccess, FErr>
where
    FSuccess: Fn(AuthenticationResult),
    FErr: Fn(JsValue),
{
    auth: msal::PublicClientApplication,
    on_redirect_success: FSuccess,
    on_redirect_error: FErr,
}

impl<FSuccess, FErr> PublicClientApplication for RedirectApp<FSuccess, FErr>
where
    FSuccess: Fn(AuthenticationResult),
    FErr: Fn(JsValue),
{
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl<FSuccess, FErr> RedirectApp<FSuccess, FErr>
where
    FSuccess: Fn(AuthenticationResult),
    FErr: Fn(JsValue),
{
    pub fn new(
        configuration: Configuration,
        on_redirect_success: FSuccess,
        on_redirect_error: FErr,
    ) -> Self {
        let auth = msal::PublicClientApplication::new(configuration.build());
        Self {
            auth,
            on_redirect_success,
            on_redirect_error,
        }
    }

    pub async fn login_redirect(&self) {
        self.login_redirect_with_scopes(vec![]).await
    }

    pub async fn login_redirect_with_scopes(&self, scopes: Vec<String>) {
        // Will always be ok unless the msal library errors, in which case
        // should throw anyway?
        let jsv = self.auth.handle_redirect_promise().await.unwrap();
        // AuthenticationResult will be undefined / null if not a redirect
        let auth_res = jsv.unchecked_into::<msal::AuthenticationResult>();
        if auth_res.is_undefined() || auth_res.is_null() {
            self.auth.login_redirect(scopes.into())
        } else {
            (self.on_redirect_success)(auth_res.into())
        }
    }

    pub async fn acquire_token_redirect(&self, request: RedirectRequest) {
        self.auth.acquire_token_redirect(request.into())
    }

    pub async fn sso_silent(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        sso_silent(&self.auth, request).await
    }

    pub async fn acquire_token_silent(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        acquire_token_silent(&self.auth, request).await
    }
}
pub struct PopupApp {
    auth: msal::PublicClientApplication,
}

impl PublicClientApplication for PopupApp {
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl PopupApp {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            auth: msal::PublicClientApplication::new(configuration.build()),
        }
    }

    pub async fn login_popup(&self) -> Result<AuthenticationResult, JsValue> {
        self.auth
            .login_popup(Self::empty_request())
            .await
            .map(Into::into)
    }

    pub async fn login_popup_with_scopes(
        &self,
        scopes: Vec<String>,
    ) -> Result<AuthenticationResult, JsValue> {
        self.auth.login_popup(scopes.into()).await.map(Into::into)
    }

    pub async fn sso_silent(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        sso_silent(&self.auth, request).await
    }

    pub async fn acquire_token_silent(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        acquire_token_silent(&self.auth, request).await
    }

    pub async fn acquire_token_popup(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        self.auth
            .acquire_token_popup(request.into())
            .await
            .map(Into::into)
    }
}

impl<'a> From<&'a str> for PopupApp {
    fn from(client_id: &'a str) -> Self {
        Self::new(client_id.into())
    }
}

impl<'a> From<Configuration> for PopupApp {
    fn from(configuration: Configuration) -> Self {
        PopupApp::new(configuration)
    }
}

impl<'a> From<BrowserAuthOptions> for PopupApp {
    fn from(browser_auth_options: BrowserAuthOptions) -> Self {
        Configuration::new(browser_auth_options).into()
    }
}

pub struct AccountInfo {
    pub home_account_id: String,
    pub environment: String,
    pub tenant_id: String,
    pub username: String,
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

impl AccountInfo {
    pub fn from_array(array: Array) -> Vec<Self> {
        array
            .iter()
            .map(|v| v.unchecked_into::<msal::AccountInfo>().into())
            .collect()
    }
}

#[cfg(test)]
mod tests_in_browser {

    use crate::*;
    use js_sys::Map;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    const CLIENT_ID: &str = "MY_CLIENT_ID";
    const AUTHORITY: &str = "MY_AUTHORITY";
    const REDIRECT_URI: &str = "MY_REDIRECT_URI";

    // const CLIENT_ID: &str = "3fba556e-5d4a-48e3-8e1a-fd57c12cb82e";
    // const AUTHORITY: &str = "https://login.windows-ppe.net/common/";

    #[wasm_bindgen_test]
    fn build_pub_client_full() {
        let b = BrowserAuthOptions::new(CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let c = Configuration::new(b);
        let client_app = PopupApp::new(c);
        assert_eq!(client_app.client_id(), CLIENT_ID);
        assert_eq!(client_app.authority(), AUTHORITY);
        assert_eq!(client_app.redirect_uri(), REDIRECT_URI);
    }

    #[wasm_bindgen_test]
    fn build_pub_client_from_config() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = PopupApp::from(config);
        assert_eq!(client_app.client_id(), CLIENT_ID);
    }

    #[wasm_bindgen_test]
    fn build_pub_client_from_string() {
        let client_app = PopupApp::from(CLIENT_ID);
        assert_eq!(client_app.client_id(), CLIENT_ID);
    }

    // How to correcly test these? Since require user input...
    // supress the warning for now
    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    async fn login_popup() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = PopupApp::from(config);
        client_app.login_popup();
    }

    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    fn login_redirect() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = RedirectApp::new(config, |_| (), |_| ());
        client_app.login_redirect();
    }

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

    // This is here since i discovered that Key and Value are strangely switched on a Map.foreach in Js land
    #[wasm_bindgen_test]
    fn make_hashmap_from_map() {
        let kv = ("claim key".to_string(), "claim value".to_string());
        let js_map = Map::new();
        js_map.set(&kv.0.into(), &kv.1.into());
        let js_hash_map = JsHashMapStringString::from(("claim key", "claim value"));
        let js_map_wasm: JsHashMapStringString = JsHashMapStringString::from(js_map);
        assert_eq!(js_map_wasm, js_hash_map)
    }

    // #[wasm_bindgen_test]
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
