//! In msal-browser not all the types are exported - they are just defined in as type aliases in the
//! typescript and do not have a constructor etc. in the final index.js file.
//! Writing in Rust, exporting, then calling from js fails as the Object.assign() methods used by the msal library don't work:
//! https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign
//! The properties must both be enumerable & and be owned by the object (not inherited).
//! Since the wasm object are pointers these methods fail.
//! These are the exports:
//! ```js
//! export { AuthError, AuthErrorMessage, AuthenticationResult, BrowserAuthError, BrowserAuthErrorMessage,
//! BrowserConfigurationAuthError, BrowserConfigurationAuthErrorMessage, InteractionRequiredAuthError,
//! LogLevel, Logger, PublicClientApplication };
//! ```

use js_sys::{Array, Date, Function, JsString, Map, Object};
use std::borrow::Cow;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub trait Msal {
    fn auth(&self) -> &PublicClientApplication;
}

// TODO: Use a macro rules for the extern binding generation?

#[wasm_bindgen(module = "/js/msal-browser-gobblefunk.js")]
extern "C" {
    // file://./../node_modules/@azure/msal-browser/dist/config/Configuration.d.ts
    pub type BrowserAuthOptions;

    #[wasm_bindgen(constructor)]
    pub fn new(client_id: &str) -> BrowserAuthOptions;

    #[wasm_bindgen(method, getter = clientId)]
    pub fn client_id(this: &BrowserAuthOptions) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &BrowserAuthOptions, authority: &str);

    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &BrowserAuthOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = knownAuthorities)]
    pub fn set_known_authorities(this: &BrowserAuthOptions, known_authorities: Array);

    #[wasm_bindgen(method, getter = knownAuthorities)]
    pub fn known_authorities(this: &BrowserAuthOptions) -> Option<Array>;

    #[wasm_bindgen(method, setter =redirectUri)]
    pub fn set_cloud_discovery_metadata(this: &BrowserAuthOptions, cloud_discovery_metadata: &str);

    #[wasm_bindgen(method, getter =redirectUri)]
    pub fn cloud_discovery_metadata(this: &BrowserAuthOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(this: &BrowserAuthOptions, redirect_uri: &str);

    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(this: &BrowserAuthOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = postLogoutRedirectUri)]
    pub fn set_post_logout_redirect_uri(this: &BrowserAuthOptions, post_logout_redirect_uri: &str);

    #[wasm_bindgen(method, getter = postLogoutRedirectUri)]
    pub fn post_logout_redirect_uri(this: &BrowserAuthOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = navigateToLoginRequestUrl)]
    pub fn set_navigate_to_login_request_url(
        this: &BrowserAuthOptions,
        navigate_to_login_request_url: bool,
    );

    #[wasm_bindgen(method, getter = navigateToLoginRequestUrl)]
    pub fn navigate_to_login_request_url(this: &BrowserAuthOptions) -> Option<bool>;

    pub type CacheOptions;

    #[wasm_bindgen(constructor)]
    pub fn new() -> CacheOptions;

    #[wasm_bindgen(method, setter = cacheLocation)]
    pub fn set_cache_location(this: &CacheOptions, cache_location: &str);

    #[wasm_bindgen(method, getter = cacheLocation)]
    pub fn cache_location(this: &CacheOptions) -> Option<String>;

    #[wasm_bindgen(method, setter = storeAuthStateInCookie)]
    pub fn set_store_auth_state_in_cookie(this: &CacheOptions, store_auth_state_in_cookie: bool);

    #[wasm_bindgen(method, getter = storeAuthStateInCookie)]
    pub fn store_auth_state_in_cookie(this: &CacheOptions) -> Option<bool>;

    pub type BrowserSystemOptions;

    #[wasm_bindgen(constructor)]
    pub fn new() -> BrowserSystemOptions;

    #[wasm_bindgen(method, setter = loggerOptions)]
    pub fn set_logger_options(this: &BrowserSystemOptions, logger_options: LoggerOptions);

    #[wasm_bindgen(method, getter = loggerOptions)]
    pub fn logger_options(this: &BrowserSystemOptions) -> Option<LoggerOptions>;

    #[wasm_bindgen(method, setter = tokenRenewalOffsetSeconds)]
    pub fn set_token_renewal_offset_seconds(this: &BrowserSystemOptions, logger_options: u32);

    #[wasm_bindgen(method, getter = tokenRenewalOffsetSeconds)]
    pub fn token_renewal_offset_seconds(this: &BrowserSystemOptions) -> Option<u32>;

    #[wasm_bindgen(method, setter = windowHashTimeout)]
    pub fn set_window_hash_timeout(this: &BrowserSystemOptions, window_hash_timeout: u32);

    #[wasm_bindgen(method, getter = windowHashTimeout)]
    pub fn window_hash_timeout(this: &BrowserSystemOptions) -> Option<u32>;

    #[wasm_bindgen(method, setter = iframeHashTimeout)]
    pub fn set_iframe_hash_timeout(this: &BrowserSystemOptions, iframe_hash_timeout: u32);

    #[wasm_bindgen(method, getter = iframeHashTimeout)]
    pub fn iframe_hash_timeout(this: &BrowserSystemOptions) -> Option<u32>;

    #[wasm_bindgen(method, setter = loadFrameTimeout)]
    pub fn set_load_frame_timeout(this: &BrowserSystemOptions, load_frame_timeout: u32);

    #[wasm_bindgen(method, getter = loadFrameTimeout)]
    pub fn load_frame_timeout(this: &BrowserSystemOptions) -> Option<u32>;

    // file://./../node_modules/@azure/msal-browser/dist/config/Configuration.d.ts
    pub type Configuration;

    #[wasm_bindgen(constructor)]
    pub fn new(browser_auth_options: &BrowserAuthOptions) -> Configuration;

    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &Configuration) -> BrowserAuthOptions;

    #[wasm_bindgen(method, setter)]
    pub fn set_auth(this: &Configuration, browser_auth_options: BrowserAuthOptions);

    #[wasm_bindgen(method, getter)]
    pub fn cache(this: &Configuration) -> Option<CacheOptions>;

    #[wasm_bindgen(method, setter)]
    pub fn set_cache(this: &Configuration, cache_options: CacheOptions);

    #[wasm_bindgen(method, getter)]
    pub fn system(this: &Configuration) -> Option<BrowserSystemOptions>;

    #[wasm_bindgen(method, setter)]
    pub fn set_system(this: &Configuration, system_options: BrowserSystemOptions);

    // file://./..//node_modules/@azure/msal-common/dist/request/AuthorizationUrlRequest.d.ts
    // just add the BaseRequest properties for now
    pub type AuthorizationUrlRequest;

    #[wasm_bindgen(constructor)]
    pub fn new(scopes: &Array) -> AuthorizationUrlRequest;

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &AuthorizationUrlRequest, authority: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = correlationId)]
    pub fn correlation_id(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = correlationId)]
    pub fn set_correlation_id(this: &AuthorizationUrlRequest, correlation_id: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(this: &AuthorizationUrlRequest, redirect_uri: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = extraScopesToConsent)]
    pub fn extra_scopes_to_consent(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = extraScopesToConsent)]
    pub fn set_extra_scopes_to_consent(
        this: &AuthorizationUrlRequest,
        extra_scopes_to_consent: Array,
    );

    #[cfg(test)]
    #[wasm_bindgen(method, getter = responseMode)]
    pub fn response_mode(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = responseMode)]
    pub fn set_response_mode(this: &AuthorizationUrlRequest, response_mode: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = codeChallenge)]
    pub fn code_challenge(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = codeChallenge)]
    pub fn set_code_challenge(this: &AuthorizationUrlRequest, code_challenge: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = codeChallengeMethod)]
    pub fn code_challenge_method(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = codeChallengeMethod)]
    pub fn set_code_challenge_method(this: &AuthorizationUrlRequest, code_challenge_method: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn state(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_state(this: &AuthorizationUrlRequest, state: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &AuthorizationUrlRequest, prompt: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = loginHint)]
    pub fn login_hint(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = loginHint)]
    pub fn set_login_hint(this: &AuthorizationUrlRequest, login_hint: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = domainHint)]
    pub fn domain_hint(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter = domainHint)]
    pub fn set_domain_hint(this: &AuthorizationUrlRequest, domain_hint: &str);

    // Hashmap<&str, &str>
    #[cfg(test)]
    #[wasm_bindgen(method, getter = extraQueryParameters)]
    pub fn extra_query_parameters(this: &AuthorizationUrlRequest) -> Object;

    #[wasm_bindgen(method, setter = extraQueryParameters)]
    pub fn set_extra_query_parameters(this: &AuthorizationUrlRequest, claims: Map);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn claims(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_claims(this: &AuthorizationUrlRequest, claims: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &AuthorizationUrlRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_nonce(this: &AuthorizationUrlRequest, nonce: &str);

    // file://./..//node_modules/@azure/msal-common/dist/account/AccountInfo.d.ts
    pub type AccountInfo;

    #[wasm_bindgen(constructor)]
    pub fn new(
        home_account_id: &str,
        environment: &str,
        tenant_id: &str,
        username: &str,
    ) -> AccountInfo;

    #[wasm_bindgen(method, getter = homeAccountId)]
    pub fn home_account_id(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn environment(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter = tenantId)]
    pub fn tenant_id(this: &AccountInfo) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn username(this: &AccountInfo) -> String;

    // file://./..//node_modules/@azure/msal-common/dist/request/EndSessionRequest.d.ts
    pub type EndSessionRequest;

    #[wasm_bindgen(constructor)]
    pub fn new() -> EndSessionRequest;

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn account(this: &EndSessionRequest) -> Option<AccountInfo>;

    #[wasm_bindgen(method, setter)]
    pub fn set_account(this: &EndSessionRequest, account: AccountInfo);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = postLogoutRedirectUri)]
    pub fn post_logout_redirect_uri(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter = postLogoutRedirectUri)]
    pub fn set_post_logout_redirect_uri(this: &EndSessionRequest, post_logout_redirect_uri: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &EndSessionRequest, authority: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = correlationId)]
    pub fn correlation_id(this: &EndSessionRequest) -> Option<String>;

    #[wasm_bindgen(method, setter = correlationId)]
    pub fn set_correlation_id(this: &EndSessionRequest, correlation_id: &str);

    // file://./..//node_modules/@azure/msal-browser/dist/request/RedirectRequest.d.ts
    pub type RedirectRequest;

    #[wasm_bindgen(constructor)]
    pub fn new(scopes: &Array) -> RedirectRequest;

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_authority(this: &RedirectRequest, authority: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = correlationId)]
    pub fn correlation_id(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = correlationId)]
    pub fn set_correlation_id(this: &RedirectRequest, correlation_id: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(this: &RedirectRequest, redirect_uri: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = extraScopesToConsent)]
    pub fn extra_scopes_to_consent(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = extraScopesToConsent)]
    pub fn set_extra_scopes_to_consent(this: &RedirectRequest, extra_scopes_to_consent: Array);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = responseMode)]
    pub fn response_mode(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = responseMode)]
    pub fn set_response_mode(this: &RedirectRequest, response_mode: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = codeChallenge)]
    pub fn code_challenge(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = codeChallenge)]
    pub fn set_code_challenge(this: &RedirectRequest, code_challenge: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = codeChallengeMethod)]
    pub fn code_challenge_method(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = codeChallengeMethod)]
    pub fn set_code_challenge_method(this: &RedirectRequest, code_challenge_method: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn state(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_state(this: &RedirectRequest, state: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn prompt(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_prompt(this: &RedirectRequest, prompt: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = loginHint)]
    pub fn login_hint(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = loginHint)]
    pub fn set_login_hint(this: &RedirectRequest, login_hint: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = domainHint)]
    pub fn domain_hint(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter = domainHint)]
    pub fn set_domain_hint(this: &RedirectRequest, domain_hint: &str);

    // Hashmap<&str, &str>
    #[cfg(test)]
    #[wasm_bindgen(method, getter = extraQueryParameters)]
    pub fn extra_query_parameters(this: &RedirectRequest) -> Object;

    #[wasm_bindgen(method, setter = extraQueryParameters)]
    pub fn set_extra_query_parameters(this: &RedirectRequest, claims: Map);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn claims(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_claims(this: &RedirectRequest, claims: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn nonce(this: &RedirectRequest) -> String;

    #[wasm_bindgen(method, setter)]
    pub fn set_nonce(this: &RedirectRequest, nonce: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = redirectStartPage)]
    pub fn redirect_start_page(this: &RedirectRequest) -> Option<String>;

    #[wasm_bindgen(method, setter = redirectStartPage)]
    pub fn set_redirect_start_page(this: &RedirectRequest, redirect_start_page: &str);

    // file://./..//node_modules/@azure/msal-browser/dist/request/SilentRequest.d.ts
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
    pub fn set_authority(request: &SilentRequest, authority: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter)]
    pub fn authority(request: &SilentRequest) -> Option<String>;

    #[wasm_bindgen(method, setter = correlationId)]
    pub fn set_correlation_id(request: &SilentRequest, correlation_id: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = correlationId)]
    pub fn correlation_id(request: &SilentRequest) -> Option<String>;

    #[wasm_bindgen(method, setter = forceRefresh)]
    pub fn set_force_refresh(request: &SilentRequest, force_refresh: bool);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = forceRefresh)]
    pub fn force_refresh(request: &SilentRequest) -> Option<bool>;

    #[wasm_bindgen(method, setter = redirectUri)]
    pub fn set_redirect_uri(request: &SilentRequest, redirect_uri: &str);

    #[cfg(test)]
    #[wasm_bindgen(method, getter = redirectUri)]
    pub fn redirect_uri(request: &SilentRequest) -> Option<String>;

    pub type LoggerOptions;

    #[wasm_bindgen(constructor)]
    pub fn new() -> LoggerOptions;

    #[wasm_bindgen(method, getter = loggerCallback)]
    pub fn logger_callback(this: &LoggerOptions) -> Option<Function>;

    #[wasm_bindgen(method, setter = loggerCallback)]
    pub fn set_logger_callback_function(this: &LoggerOptions, logger_callback: &Function);

    #[wasm_bindgen(method, setter = loggerCallback)]
    pub fn set_logger_callback(
        this: &LoggerOptions,
        logger_callback: &Closure<dyn Fn(String, String, bool)>,
    );

    #[wasm_bindgen(method, getter = piiLoggingEnabled)]
    pub fn pii_logging_enabled(this: &LoggerOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter = piiLoggingEnabled)]
    pub fn set_pii_logging_enabled(this: &LoggerOptions, pii_logging_enabled: bool);

    // Due to way enum is defined can return int or string
    #[wasm_bindgen(method, getter = logLevel)]
    pub fn log_level(this: &LoggerOptions) -> JsValue;

    #[wasm_bindgen(method, setter = logLevel)]
    pub fn set_log_level(this: &LoggerOptions, log_level: &str);

}

// file://./../node_modules/@azure/msal-browser/dist/index.js
#[wasm_bindgen(module = "/js/msal-browser.js")]
extern "C" {

    // file://./../node_modules/@azure/msal-browser/dist/app/PublicClientApplication.d.ts
    pub type PublicClientApplication;

    #[wasm_bindgen(constructor)]
    pub fn new(config: Configuration) -> PublicClientApplication;

    #[wasm_bindgen(method, getter)]
    pub fn config(this: &PublicClientApplication) -> Configuration;

    // returns an AuthenticationResult
    // have to call this on every page load and check if its null
    #[wasm_bindgen(method, js_name = handleRedirectPromise, catch)]
    pub async fn handle_redirect_promise(
        this: &PublicClientApplication,
    ) -> Result<JsValue, JsValue>;

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

    #[wasm_bindgen(method, js_name = getAccountByHomeId)]
    pub fn get_account_by_home_id(
        this: &PublicClientApplication,
        username: String,
    ) -> Option<AccountInfo>;

    // returns an AuthenticationResult
    #[wasm_bindgen(method, catch, js_name = ssoSilent, catch)]
    pub async fn sso_silent(
        this: &PublicClientApplication,
        request: AuthorizationUrlRequest,
    ) -> Result<JsValue, JsValue>;

    // returns an AuthenticationResult
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

    // file://./../node_modules/@azure/msal-common/dist/response/AuthenticationResult.d.ts
    // This is in the index, but only a constructor, so type checking fails
    pub type AuthenticationResult;

    #[wasm_bindgen(constructor)]
    pub fn new() -> AuthenticationResult;

    #[wasm_bindgen(method, getter = uniqueId)]
    pub fn unique_id(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, getter = tenantId)]
    pub fn tenant_id(this: &AuthenticationResult) -> String;

    // returns Vec<String>
    #[wasm_bindgen(method, getter)]
    pub fn scopes(this: &AuthenticationResult) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn account(this: &AuthenticationResult) -> AccountInfo;

    #[wasm_bindgen(method, getter = idToken)]
    pub fn id_token(this: &AuthenticationResult) -> String;

    // Returns Hashmap<String, String | f64> ?
    #[wasm_bindgen(method, getter = idTokenClaims)]
    pub fn id_token_claims(this: &AuthenticationResult) -> Object;

    #[wasm_bindgen(method, getter = accessToken)]
    pub fn access_token(this: &AuthenticationResult) -> String;

    #[wasm_bindgen(method, getter = fromCache)]
    pub fn from_cache(this: &AuthenticationResult) -> bool;

    #[wasm_bindgen(method, getter = expiresOn)]
    pub fn expires_on(this: &AuthenticationResult) -> Date;

    #[wasm_bindgen(method, getter = extExpiresOn)]
    pub fn ext_expires_on(this: &AuthenticationResult) -> Option<Date>;

    #[wasm_bindgen(method, getter)]
    pub fn state(this: &AuthenticationResult) -> Option<String>;

    #[wasm_bindgen(method, getter = familyId)]
    pub fn family_id(this: &AuthenticationResult) -> Option<String>;

    // file://./../node_modules/@azure/msal-common/dist/logger/Logger.d.ts
    pub type LogLevel;
}

impl<'a, T> From<&'a [T]> for AuthorizationUrlRequest
where
    T: Into<String> + Clone,
{
    fn from(scopes: &'a [T]) -> Self {
        let js: JsArrayString = scopes.into();
        AuthorizationUrlRequest::new(&js.into())
    }
}

fn array_unchecked_to_vec<JsT, T>(array: Array) -> Vec<T>
where
    JsT: JsCast + Into<T>,
{
    array
        .iter()
        .map(|v| v.unchecked_into::<JsT>().into())
        .collect()
}

/// These are so can use From<T>
pub(crate) struct JsArrayString(pub Vec<String>);

impl From<String> for JsArrayString {
    fn from(scope: String) -> Self {
        Self(vec![scope])
    }
}

impl<'a> From<&'a Vec<Cow<'a, str>>> for JsArrayString {
    fn from(scopes: &'a Vec<Cow<'a, str>>) -> Self {
        Self(scopes.clone().into_iter().map(Cow::into_owned).collect())
    }
}

impl<'a, T> From<&'a [T]> for JsArrayString
where
    T: Clone + Into<String>,
{
    fn from(scopes: &'a [T]) -> Self {
        Self(scopes.iter().cloned().map(|s| s.into()).collect())
    }
}

impl From<Array> for JsArrayString {
    fn from(array: Array) -> Self {
        Self(array_unchecked_to_vec::<JsString, String>(array))
    }
}

impl From<JsArrayString> for Array {
    fn from(js_array_string: JsArrayString) -> Self {
        js_array_string.0.into_iter().map(JsValue::from).collect()
    }
}

impl<'a> From<JsArrayString> for Vec<Cow<'a, str>> {
    fn from(js_array_string: JsArrayString) -> Self {
        js_array_string.0.into_iter().map(Cow::from).collect()
    }
}

pub struct JsHashMapStrStr<'a>(&'a HashMap<Cow<'a, str>, Cow<'a, str>>);

impl<'a> From<&'a HashMap<Cow<'a, str>, Cow<'a, str>>> for JsHashMapStrStr<'a> {
    fn from(hm: &'a HashMap<Cow<'a, str>, Cow<'a, str>>) -> Self {
        Self(hm)
    }
}

impl<'a> From<JsHashMapStrStr<'a>> for Map {
    fn from(map: JsHashMapStrStr<'a>) -> Self {
        let js = Map::new();
        for (k, v) in map.0 {
            js.set(&(**k).into(), &(**v).into());
        }
        js
    }
}
