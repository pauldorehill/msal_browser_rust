//! In msal-browser not all the types are exported - they are just defined in as type aliases in the
//! typescript and do not have a constructor etc. in the final index.js file.
//! Writing in Rust, exporting, then calling from js fails as the Object.assign() methods used by the msal library don't work:
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign
//! The properties must both be enumerable & and be owned by the object (not inherited).
//! Since the wasm object are pointers these methods fail.
//! These are the exports:
//! ```js
//! exports.AuthError = AuthError;
//! exports.AuthErrorMessage = AuthErrorMessage;
//! exports.AuthenticationResult = AuthenticationResult;
//! exports.BrowserAuthError = BrowserAuthError;
//! exports.BrowserAuthErrorMessage = BrowserAuthErrorMessage;
//! exports.BrowserConfigurationAuthError = BrowserConfigurationAuthError;
//! exports.BrowserConfigurationAuthErrorMessage = BrowserConfigurationAuthErrorMessage;
//! exports.InteractionRequiredAuthError = InteractionRequiredAuthError;
//! exports.Logger = Logger;
//! exports.PublicClientApplication = PublicClientApplication;
//! ```

use js_sys::{Array, Date, JsString, Map};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub trait JsMirror: std::marker::Sized {
    type JsTarget: From<Self>;
}

#[wasm_bindgen(module = "/msal-browser-gobblefunk.js")]
extern "C" {

    // file://./../node_modules/@azure/msal-browser/dist/src/config/Configuration.d.ts
    pub type BrowserAuthOptions;

    #[wasm_bindgen(constructor)]
    pub fn new(client_id: &str) -> BrowserAuthOptions;

    #[wasm_bindgen(method, getter = clientId)]
    pub fn client_id(this: &BrowserAuthOptions) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &BrowserAuthOptions, authority: &str);

    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &BrowserAuthOptions) -> String;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(this: &BrowserAuthOptions, redirect_uri: &str);

    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(this: &BrowserAuthOptions) -> String;

    // file://./../node_modules/@azure/msal-browser/dist/src/config/Configuration.d.ts
    pub type Configuration;

    #[wasm_bindgen(constructor)]
    pub fn new(browser_auth_options: &BrowserAuthOptions) -> Configuration;

    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &Configuration) -> BrowserAuthOptions;

    // file://./..//node_modules/@azure/msal-common/dist/src/request/AuthorizationUrlRequest.d.ts
    // just add the BaseRequest properties for now
    pub type AuthorizationUrlRequest;

    #[wasm_bindgen(constructor)]
    pub fn new(scopes: &Array) -> AuthorizationUrlRequest;

    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &AuthorizationUrlRequest, authority: String);

    #[wasm_bindgen(method, getter, js_name = correlationId)]
    pub fn correlation_id(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter, js_name = correlationId)]
    pub fn set_correlation_id(this: &AuthorizationUrlRequest, correlation_id: String);

    // file://./..//node_modules/@azure/msal-common/dist/src/account/AccountInfo.d.ts
    pub type AccountInfo;

    #[wasm_bindgen(constructor)]
    pub fn new(
        homeAccountId: String,
        environment: String,
        tenantId: String,
        username: String,
    ) -> AccountInfo;

    #[wasm_bindgen(method, getter, js_name = homeAccountId)]
    pub fn home_account_id(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn environment(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter, js_name = tenantId)]
    pub fn tenant_id(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn username(this: &AccountInfo) -> String;

    // file://./..//node_modules/@azure/msal-common/dist/src/request/EndSessionRequest.d.ts
    pub type EndSessionRequest;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EndSessionRequest;

    #[wasm_bindgen(method, getter)]
    pub fn account(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_account(this: &EndSessionRequest, account: String);

    #[wasm_bindgen(method, getter, js_name = postLogoutRedirectUri)]
    pub fn post_logout_redirect_uri(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = postLogoutRedirectUri)]
    pub fn set_post_logout_redirect_uri(this: &EndSessionRequest, post_logout_redirect_uri: String);

    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &EndSessionRequest, authority: String);

    #[wasm_bindgen(method, getter, js_name = correlationId)]
    pub fn correlation_id(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = correlationId)]
    pub fn set_correlation_id(this: &EndSessionRequest, correlation_id: String);

    // file://./..//node_modules/@azure/msal-browser/dist/src/request/RedirectRequest.d.ts
    pub type RedirectRequest;

    #[wasm_bindgen(constructor)]
    pub fn new(scopes: &Array) -> RedirectRequest;

    // file://./..//node_modules/@azure/msal-browser/dist/src/request/SilentRequest.d.ts
    pub type SilentRequest;

    #[wasm_bindgen(constructor)]
    pub fn new(scopes: &Array, account: AccountInfo) -> SilentRequest;

}

// file://./../node_modules/@azure/msal-browser/dist/index.es.js
// Copied locally in a build script
#[wasm_bindgen(module = "/msal-browser.js")]
extern "C" {

    //file://./../node_modules/@azure/msal-browser/dist/src/app/PublicClientApplication.d.ts
    pub type PublicClientApplication;

    #[wasm_bindgen(constructor)]
    pub fn new(config: Configuration) -> PublicClientApplication;

    #[wasm_bindgen(method, getter)]
    pub fn config(this: &PublicClientApplication) -> Configuration;

    /// returns an AuthenticationResult
    /// have to call this on every page load and check if its null
    #[wasm_bindgen(method, js_name = handleRedirectPromise, catch)]
    pub async fn handle_redirect_promise(this: &PublicClientApplication) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = loginPopup, catch)]
    pub async fn login_popup(
        this: &PublicClientApplication,
        login_request: AuthorizationUrlRequest,
    ) -> Result<JsValue, JsValue>;

    // In the ts file this is marked as a Promise<void> and should not be awaited since navigating away
    #[wasm_bindgen(method, js_name = loginRedirect)]
    pub fn login_redirect(this: &PublicClientApplication, request: AuthorizationUrlRequest);

    #[wasm_bindgen(method)]
    pub fn logout(this: &PublicClientApplication, request: EndSessionRequest);

    // returns [AccountInfo]
    #[wasm_bindgen(method, js_name = getAllAccounts)]
    pub fn get_all_accounts(this: &PublicClientApplication) -> Array;

    #[wasm_bindgen(method, js_name = getAccountByUsername)]
    pub fn get_account_by_username(this: &PublicClientApplication, username: String)
        -> AccountInfo;

    /// returns an AuthenticationResult
    #[wasm_bindgen(method, catch, js_name = ssoSilent, catch)]
    pub async fn sso_silent(
        this: &PublicClientApplication,
        request: AuthorizationUrlRequest,
    ) -> Result<JsValue, JsValue>;

    /// returns an AuthenticationResult
    #[wasm_bindgen(method, catch, js_name = acquireTokenSilent, catch)]
    pub async fn acquire_token_silent(
        this: &PublicClientApplication,
        request: SilentRequest,
    ) -> Result<JsValue, JsValue>;

    // In the ts file this is marked as a Promise<void> and should not be awaited since navigating away
    #[wasm_bindgen(method, js_name = acquireTokenRedirect)]
    pub fn acquire_token_redirect(this: &PublicClientApplication, request: RedirectRequest);

    // returns an AuthenticationResult
    #[wasm_bindgen(method, js_name = acquireTokenPopup, catch)]
    pub async fn acquire_token_popup(
        this: &PublicClientApplication,
        request: AuthorizationUrlRequest,
    ) -> Result<JsValue, JsValue>;

    //file://./../node_modules/@azure/msal-browser/dist/src/app/PublicClientApplication.d.ts
    // This is in the index, but only a constructor, so type checking fails
    pub type AuthenticationResult;

    #[wasm_bindgen(constructor)]
    pub fn new() -> AuthenticationResult;

    #[wasm_bindgen(method, getter, js_name = uniqueId)]
    pub fn unique_id(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, setter, js_name = uniqueId)]
    pub fn set_unique_id(this: &AuthenticationResult, unique_id: String);

    #[wasm_bindgen(method, getter, js_name = tenantId)]
    pub fn tenant_id(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, setter, js_name = tenantId)]
    pub fn set_tenant_id(this: &AuthenticationResult, tenant_id: String);

    /// returns Vec<String>
    #[wasm_bindgen(method, getter)]
    pub fn scopes(this: &AuthenticationResult) -> Array;

    #[wasm_bindgen(method, setter)]
    pub fn set_scopes(this: &AuthenticationResult, scopes: Array);

    #[wasm_bindgen(method, getter)]
    pub fn account(this: &AuthenticationResult) -> AccountInfo;

    #[wasm_bindgen(method, setter)]
    pub fn set_account(this: &AuthenticationResult, account_info: AccountInfo);

    #[wasm_bindgen(method, getter, js_name = idToken)]
    pub fn id_token(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, setter, js_name = idToken)]
    pub fn set_id_token(this: &AuthenticationResult, id_token: String);

    /// TODO: This is throwing so i need to sort
    /// Returns Hashmap<String, String>
    /// When the json comes back parsing as map fails on having none of the
    /// enumerable methods
    /// JS exception that was thrown: TypeError: getObject(...).forEach is not a function
    #[wasm_bindgen(method, getter, js_name = idTokenClaims)]
    pub fn id_token_claims(this: &AuthenticationResult) -> Map;

    /// takes a Hashmap<String, String>
    #[wasm_bindgen(method, setter, js_name = idTokenClaims)]
    pub fn set_id_token_claims(this: &AuthenticationResult, id_token_claims: Map);

    #[wasm_bindgen(method, getter, js_name = accessToken)]
    pub fn access_token(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, setter, js_name = accessToken)]
    pub fn set_access_token(this: &AuthenticationResult, access_token: String);

    #[wasm_bindgen(method, getter, js_name = fromCache)]
    pub fn from_cache(this: &AuthenticationResult) -> bool;

    #[wasm_bindgen(method, setter, js_name = fromCache)]
    pub fn set_from_cache(this: &AuthenticationResult, from_cache: bool);

    #[wasm_bindgen(method, getter, js_name = expiresOn)]
    pub fn expires_on(this: &AuthenticationResult) -> Date;

    #[wasm_bindgen(method, setter, js_name = expiresOn)]
    pub fn set_expires_on(this: &AuthenticationResult, date: Date);

    #[wasm_bindgen(method, getter, js_name = extExpiresOn)]
    pub fn ext_expires_on(this: &AuthenticationResult) -> Option<Date>;

    #[wasm_bindgen(method, setter, js_name = extExpiresOn)]
    pub fn set_ext_expires_on(this: &AuthenticationResult, date: Option<Date>);

    #[wasm_bindgen(method, getter)]
    pub fn state(this: &AuthenticationResult) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_state(this: &AuthenticationResult, state: Option<String>);

    #[wasm_bindgen(method, getter, js_name = familyId)]
    pub fn family_id(this: &AuthenticationResult) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = familyId)]
    pub fn set_family_id(this: &AuthenticationResult, family_id: Option<String>);
}

/// Here to allow passing in a scopes array on the login request
impl From<Vec<String>> for AuthorizationUrlRequest {
    fn from(scopes: Vec<String>) -> Self {
        let js: JsArrayString = scopes.into();
        AuthorizationUrlRequest::new(&js.into())
    }
}

// TODO: Should i be using unchecked? I know the types, would likely
// just unwrap anyway... but could pass out more usefull error?
// Do i need both generics?
// Using newtype pattern to allow use of traits

fn array_to_vec<JsT, FinalT>(array: Array) -> Vec<FinalT>
where
    JsT: JsCast + Into<FinalT>,
{
    array
        .iter()
        .map(|v| v.unchecked_into::<JsT>().into())
        .collect()
}

/// These are so can use From<T>
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct JsArrayString(pub Vec<String>);

impl From<String> for JsArrayString {
    fn from(scope: String) -> Self {
        Self(vec![scope])
    }
}

impl From<&str> for JsArrayString {
    fn from(scope: &str) -> Self {
        vec![scope.to_string()].into()
    }
}

impl From<Vec<String>> for JsArrayString {
    fn from(xs: Vec<String>) -> Self {
        JsArrayString(xs)
    }
}

impl From<Array> for JsArrayString {
    fn from(array: Array) -> Self {
        Self(array_to_vec::<JsString, String>(array))
    }
}

impl From<JsArrayString> for Array {
    fn from(js_array_string: JsArrayString) -> Self {
        js_array_string
            .0
            .into_iter()
            .map(|s| JsValue::from(s))
            .collect()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct JsHashMapStringString(pub HashMap<String, String>);

impl From<Map> for JsHashMapStringString {
    fn from(js_map: Map) -> Self {
        let mut hm = HashMap::new();
        // Value and Key are swapped in Js land!
        // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach
        js_map.for_each(&mut |v, k| {
            hm.insert(
                k.unchecked_into::<JsString>().into(),
                v.unchecked_into::<JsString>().into(),
            );
        });
        JsHashMapStringString(hm)
    }
}

impl From<JsHashMapStringString> for Map {
    fn from(hm: JsHashMapStringString) -> Self {
        let js_hm = Map::new();
        for (k, v) in hm.0 {
            js_hm.set(&k.into(), &v.into());
        }
        js_hm
    }
}

#[cfg(test)]
impl From<HashMap<String, String>> for JsHashMapStringString {
    fn from(hm: HashMap<String, String>) -> Self {
        Self(hm)
    }
}

#[cfg(test)]
impl From<(&str, &str)> for JsHashMapStringString {
    fn from(kv: (&str, &str)) -> Self {
        let mut hm = HashMap::new();
        hm.insert(kv.0.to_string(), kv.1.to_string());
        Self(hm)
    }
}
