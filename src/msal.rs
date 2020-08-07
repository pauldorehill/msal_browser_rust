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

use crate::TokenValue;
use js_sys::{Array, Date, JsString, Map, Object};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub trait JsMirror: std::marker::Sized {
    // TODO: Toggle this on?
    type JsTarget: From<Self>; // + Into<Self>;
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
    pub fn authority(this: &BrowserAuthOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(this: &BrowserAuthOptions, redirect_uri: &str);

    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(this: &BrowserAuthOptions) -> Option<String>;

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

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &AuthorizationUrlRequest, authority: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = correlationId)]
    pub fn correlation_id(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter, js_name = correlationId)]
    pub fn set_correlation_id(this: &AuthorizationUrlRequest, correlation_id: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = loginHint)]
    pub fn login_hint(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter, js_name = loginHint)]
    pub fn set_login_hint(this: &AuthorizationUrlRequest, login_hint: String);

    // file://./..//node_modules/@azure/msal-common/dist/src/account/AccountInfo.d.ts
    pub type AccountInfo;

    #[wasm_bindgen(constructor)]
    pub fn new(
        home_account_id: String,
        environment: String,
        tenant_id: String,
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

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn account(this: &EndSessionRequest) -> Option<AccountInfo>;

    #[wasm_bindgen(method, setter)]
    pub fn set_account(this: &EndSessionRequest, account: AccountInfo);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = postLogoutRedirectUri)]
    pub fn post_logout_redirect_uri(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = postLogoutRedirectUri)]
    pub fn set_post_logout_redirect_uri(this: &EndSessionRequest, post_logout_redirect_uri: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &EndSessionRequest, authority: String);

    #[cfg(test)]
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

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn scopes(request: &SilentRequest) -> Array;

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn account(request: &SilentRequest) -> AccountInfo;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(request: &SilentRequest, authority: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(request: &SilentRequest) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = correlationId)]
    pub fn set_correlation_id(request: &SilentRequest, correlation_id: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = correlationId)]
    pub fn correlation_id(request: &SilentRequest) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = forceRefresh)]
    pub fn set_force_refresh(request: &SilentRequest, force_refresh: bool);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = forceRefresh)]
    pub fn force_refresh(request: &SilentRequest) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = redirectUri)]
    pub fn set_redirect_uri(request: &SilentRequest, redirect_uri: String);

    #[cfg(test)]
    #[wasm_bindgen(method, getter, js_name = redirectUri)]
    pub fn redirect_uri(request: &SilentRequest,) -> Option<String>;

}

// file://./../node_modules/@azure/msal-browser/dist/index.es.js
#[wasm_bindgen(module = "/node_modules/@azure/msal-browser/dist/index.es.js")]
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
    pub fn get_all_accounts(this: &PublicClientApplication) -> Option<Array>;

    #[wasm_bindgen(method, js_name = getAccountByUsername)]
    pub fn get_account_by_username(
        this: &PublicClientApplication,
        username: String,
    ) -> Option<AccountInfo>;

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

    #[wasm_bindgen(method, getter, js_name = tenantId)]
    pub fn tenant_id(this: &AuthenticationResult) -> String;

    /// returns Vec<String>
    #[wasm_bindgen(method, getter)]
    pub fn scopes(this: &AuthenticationResult) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn account(this: &AuthenticationResult) -> AccountInfo;

    #[wasm_bindgen(method, getter, js_name = idToken)]
    pub fn id_token(this: &AuthenticationResult) -> String;

    /// Returns Hashmap<String, String | f64> ?
    #[wasm_bindgen(method, getter, js_name = idTokenClaims)]
    pub fn id_token_claims(this: &AuthenticationResult) -> Object;

    #[wasm_bindgen(method, getter, js_name = accessToken)]
    pub fn access_token(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, getter, js_name = fromCache)]
    pub fn from_cache(this: &AuthenticationResult) -> bool;

    #[wasm_bindgen(method, getter, js_name = expiresOn)]
    pub fn expires_on(this: &AuthenticationResult) -> Date;

    #[wasm_bindgen(method, getter, js_name = extExpiresOn)]
    pub fn ext_expires_on(this: &AuthenticationResult) -> Option<Date>;

    #[wasm_bindgen(method, getter)]
    pub fn state(this: &AuthenticationResult) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = familyId)]
    pub fn family_id(this: &AuthenticationResult) -> Option<String>;
}

/// Here to allow passing in a scopes array on the login request
impl From<Vec<String>> for AuthorizationUrlRequest {
    fn from(scopes: Vec<String>) -> Self {
        let js: JsArrayString = scopes.into();
        AuthorizationUrlRequest::new(&js.into())
    }
}

// TODO: Should i be using unchecked? I know the types, would likely
// just unwrap anyway... but could pass out more useful error?
fn array_to_vec<JsT, T>(array: Array) -> Vec<T>
where
    JsT: JsCast + Into<T>,
{
    array
        .iter()
        .map(|v| v.unchecked_into::<JsT>().into())
        .collect()
}

/// These are so can use From<T>
#[derive(Clone)]
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
        js_array_string.0.into_iter().map(JsValue::from).collect()
    }
}

#[derive(Clone)]
pub(crate) struct TokenHashMap(pub HashMap<String, TokenValue>);

impl From<Object> for TokenHashMap {
    fn from(js_obj: Object) -> Self {
        let mut hm = HashMap::new();
        js_sys::Object::entries(&js_obj).for_each(&mut |v, _, _| {
            let kv = v.unchecked_into::<Array>();
            // Returned keys are always strings
            let key = kv.get(0).unchecked_into::<JsString>().into();
            let value = {
                let v = kv.get(1);
                match v.as_string() {
                    Some(s) => TokenValue::String(s),
                    None => TokenValue::Float(v.as_f64().unwrap()),
                }
            };
            hm.insert(key, value);
        });
        TokenHashMap(hm)
    }
}

impl From<TokenHashMap> for Map {
    fn from(map: TokenHashMap) -> Self {
        let js_map = Map::new();
        for (k, v) in map.0 {
            let v = match v {
                TokenValue::String(s) => s.into(),
                TokenValue::Float(f) => f.into(),
            };
            js_map.set(&k.into(), &v);
        }
        js_map
    }
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen(module = "/msal-object-examples.js")]
    extern "C" {
        static accessToken: Object;
        static idToken: Object;
    }

    #[wasm_bindgen_test]
    fn parse_access_token() {
        let _: TokenHashMap = accessToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn parse_id_token() {
        let _: TokenHashMap = idToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn test_parsing_key_values() {
        let hm: TokenHashMap = idToken.clone().into();
        assert_eq!(hm.0.get("typ").unwrap(), &TokenValue::String("JWT".into()))
    }
}
