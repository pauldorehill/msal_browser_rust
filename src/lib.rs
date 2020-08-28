//! msal-browser.js in Rust WASM
mod msal;

// TODO: Many uses of unchecked_into... might be better to do something else:
// Maybe consider https://docs.rs/js-sys/0.3.44/js_sys/Reflect/index.html
// https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html

#[cfg(feature = "popup")]
pub mod popup;
#[cfg(feature = "redirect")]
pub mod redirect;
pub mod requests;

use js_sys::{Array, Date, Function, Object};
use msal::JsArrayString;
use requests::*;
use std::borrow::{Borrow, Cow};
use std::convert::{TryFrom, TryInto};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

pub struct BrowserAuthOptions<'a> {
    client_id: Cow<'a, str>,
    authority: Option<Cow<'a, str>>,
    known_authorities: Option<Vec<Cow<'a, str>>>,
    cloud_discovery_metadata: Option<Cow<'a, str>>,
    redirect_uri: Option<Cow<'a, str>>,
    post_logout_redirect_uri: Option<Cow<'a, str>>,
    navigate_to_login_request_url: Option<bool>,
}

impl<'a> From<BrowserAuthOptions<'a>> for msal::BrowserAuthOptions {
    fn from(auth_options: BrowserAuthOptions) -> Self {
        let auth = msal::BrowserAuthOptions::new(&auth_options.client_id);

        if let Some(v) = &auth_options.authority {
            auth.set_authority(v);
        }
        if let Some(v) = &auth_options.known_authorities {
            auth.set_known_authorities(JsArrayString::from(v).into())
        }
        if let Some(v) = &auth_options.cloud_discovery_metadata {
            auth.set_cloud_discovery_metadata(v)
        }
        if let Some(v) = &auth_options.redirect_uri {
            auth.set_redirect_uri(v)
        }
        if let Some(v) = &auth_options.post_logout_redirect_uri {
            auth.set_post_logout_redirect_uri(v)
        }
        if let Some(v) = &auth_options.navigate_to_login_request_url {
            auth.set_navigate_to_login_request_url(*v)
        }
        auth
    }
}

impl<'a> From<msal::BrowserAuthOptions> for BrowserAuthOptions<'a> {
    fn from(auth: msal::BrowserAuthOptions) -> Self {
        Self {
            client_id: Cow::from(auth.client_id()),
            authority: auth.authority().map(Cow::from),
            known_authorities: auth
                .known_authorities()
                .map(|v| JsArrayString::from(v).into()),
            cloud_discovery_metadata: auth.cloud_discovery_metadata().map(Cow::from),
            redirect_uri: auth.redirect_uri().map(Cow::from),
            post_logout_redirect_uri: auth.post_logout_redirect_uri().map(Cow::from),
            navigate_to_login_request_url: auth.navigate_to_login_request_url(),
        }
    }
}

impl<'a> BrowserAuthOptions<'a> {
    pub fn new<T>(client_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            client_id: client_id.into(),
            authority: None,
            known_authorities: None,
            cloud_discovery_metadata: None,
            redirect_uri: None,
            post_logout_redirect_uri: None,
            navigate_to_login_request_url: None,
        }
    }

    pub fn set_authority<T>(mut self, authority: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.authority = Some(authority.into());
        self
    }

    pub fn set_known_authorities<T>(mut self, known_authorities: &[T]) -> Self
    where
        T: Into<Cow<'a, str>> + std::clone::Clone,
    {
        let xs = known_authorities
            .to_vec()
            .into_iter()
            .map(|v| v.into())
            .collect();
        self.known_authorities = Some(xs);
        self
    }

    pub fn set_cloud_discovery_metadata<T>(mut self, cloud_discovery_metadata: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.cloud_discovery_metadata = Some(cloud_discovery_metadata.into());
        self
    }

    pub fn set_redirect_uri<T>(mut self, redirect_uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    pub fn set_post_logout_redirect_uri<T>(mut self, post_logout_redirect_uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.post_logout_redirect_uri = Some(post_logout_redirect_uri.into());
        self
    }

    pub fn set_navigate_to_login_request_url(
        mut self,
        navigate_to_login_request_url: bool,
    ) -> Self {
        self.navigate_to_login_request_url = Some(navigate_to_login_request_url);
        self
    }
}

pub enum CacheLocation {
    Session,
    Local,
}

impl CacheLocation {
    const SESSION: &'static str = "sessionStorage";
    const LOCAL: &'static str = "localStorage";
}

impl Borrow<str> for CacheLocation {
    fn borrow(&self) -> &str {
        match &self {
            CacheLocation::Session => Self::SESSION,
            CacheLocation::Local => Self::LOCAL,
        }
    }
}

impl TryFrom<String> for CacheLocation {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            Self::LOCAL => Ok(Self::Local),
            Self::SESSION => Ok(Self::Session),
            _ => Err("Input not valid".into()),
        }
    }
}

#[derive(Default)]
pub struct CacheOptions {
    cache_location: Option<CacheLocation>,
    store_auth_state_in_cookie: Option<bool>,
}

impl CacheOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_cache_location(mut self, cache_location: CacheLocation) -> Self {
        self.cache_location = Some(cache_location);
        self
    }

    pub fn set_store_auth_state_in_cookie(mut self, store_auth_state_in_cookie: bool) -> Self {
        self.store_auth_state_in_cookie = Some(store_auth_state_in_cookie);
        self
    }
}

impl From<CacheOptions> for msal::CacheOptions {
    fn from(cache_options: CacheOptions) -> Self {
        let cache = msal::CacheOptions::new();
        if let Some(v) = cache_options.cache_location {
            cache.set_cache_location(v.borrow())
        }
        if let Some(v) = cache_options.store_auth_state_in_cookie {
            cache.set_store_auth_state_in_cookie(v)
        }
        cache
    }
}

impl From<msal::CacheOptions> for CacheOptions {
    fn from(cache: msal::CacheOptions) -> Self {
        Self {
            cache_location: cache.cache_location().and_then(|v| v.try_into().ok()),
            store_auth_state_in_cookie: cache.store_auth_state_in_cookie(),
        }
    }
}

pub enum LogLevel {
    Error,
    Warning,
    Info,
    Verbose,
}

impl LogLevel {
    fn as_str<'a>(&self) -> &'a str {
        match self {
            LogLevel::Error => "Error",
            LogLevel::Warning => "Warning",
            LogLevel::Info => "Info",
            LogLevel::Verbose => "Verbose",
        }
    }
}

impl TryFrom<String> for LogLevel {
    type Error = ();
    fn try_from(value: String) -> Result<Self, ()> {
        value.as_str().try_into()
    }
}

impl<'a> TryFrom<&'a str> for LogLevel {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, ()> {
        match value {
            "Error" => Ok(LogLevel::Error),
            "Warning" => Ok(LogLevel::Warning),
            "Info" => Ok(LogLevel::Info),
            "Verbose" => Ok(LogLevel::Verbose),
            _ => Err(()),
        }
    }
}

impl TryFrom<f64> for LogLevel {
    type Error = ();
    fn try_from(value: f64) -> Result<Self, ()> {
        match value as u64 {
            0 => Ok(LogLevel::Error),
            1 => Ok(LogLevel::Warning),
            2 => Ok(LogLevel::Info),
            3 => Ok(LogLevel::Verbose),
            _ => Err(()),
        }
    }
}

impl TryFrom<JsValue> for LogLevel {
    type Error = JsValue;
    fn try_from(log_level: JsValue) -> Result<Self, JsValue> {
        if let Some(v) = log_level.as_string() {
            v.as_str().try_into().map_err(|_| log_level)
        } else if let Some(v) = log_level.as_f64() {
            v.try_into().map_err(|_| log_level)
        } else {
            Err(log_level)
        }
    }
}

// This is to allow loading from Js: might as well leave there
// Use of trait object reference works best as an api since no need to specify generic type
enum LoggerCallback {
    Js(Function),
    WASM(&'static dyn Fn(LogLevel, String, bool)),
}
// I think that these must use heap allocated closures since they need to be long lived
// https://rustwasm.github.io/docs/wasm-bindgen/reference/passing-rust-closures-to-js.html#heap-allocated-closures
// https://docs.rs/wasm-bindgen/0.2.67/wasm_bindgen/closure/struct.Closure.html
// Easiest to use owned values for the closure
#[derive(Default)]
pub struct LoggerOptions {
    logger_callback: Option<LoggerCallback>,
    pii_logging_enabled: Option<bool>,
    log_level: Option<LogLevel>,
}

impl LoggerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_logger_callback(
        mut self,
        logger_callback: &'static dyn Fn(LogLevel, String, bool),
    ) -> Self {
        self.logger_callback = Some(LoggerCallback::WASM(logger_callback));
        self
    }

    pub fn set_pii_logging_enabled(mut self, pii_logging_enabled: bool) -> Self {
        self.pii_logging_enabled = Some(pii_logging_enabled);
        self
    }

    pub fn set_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = Some(log_level);
        self
    }
}
impl From<LoggerOptions> for msal::LoggerOptions {
    fn from(logger: LoggerOptions) -> Self {
        let js = msal::LoggerOptions::new();
        if let Some(f) = logger.logger_callback {
            match f {
                LoggerCallback::Js(f) => js.set_logger_callback_function(&f),
                LoggerCallback::WASM(f) => {
                    let f = move |log_level: String, message: String, pii| {
                        f(log_level.try_into().unwrap(), message, pii)
                    };
                    js.set_logger_callback(&Closure::wrap(Box::new(f)));
                }
            }
        }

        if let Some(v) = logger.pii_logging_enabled {
            js.set_pii_logging_enabled(v);
        }
        if let Some(v) = logger.log_level {
            js.set_log_level(v.as_str());
        }

        js
    }
}

impl From<msal::LoggerOptions> for LoggerOptions {
    fn from(js: msal::LoggerOptions) -> Self {
        Self {
            logger_callback: js.logger_callback().map(LoggerCallback::Js),
            pii_logging_enabled: js.pii_logging_enabled(),
            log_level: js.log_level().try_into().ok(),
        }
    }
}

// TODO: Work out how to pass functions in and out: add logger options
// TODO: is u32 correct for these?
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER
#[derive(Default)]
pub struct BrowserSystemOptions {
    token_renewal_offset_seconds: Option<u32>,
    logger_options: Option<LoggerOptions>,
    window_hash_timeout: Option<u32>,
    iframe_hash_timeout: Option<u32>,
    load_frame_timeout: Option<u32>,
}

impl BrowserSystemOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_logger_options(mut self, logger_options: LoggerOptions) -> Self {
        self.logger_options = Some(logger_options);
        self
    }
    pub fn set_token_renewal_offset_seconds(mut self, token_renewal_offset_seconds: u32) -> Self {
        self.token_renewal_offset_seconds = Some(token_renewal_offset_seconds);
        self
    }

    pub fn set_window_hash_timeout(mut self, window_hash_timeout: u32) -> Self {
        self.window_hash_timeout = Some(window_hash_timeout);
        self
    }

    pub fn set_iframe_hash_timeout(mut self, iframe_hash_timeout: u32) -> Self {
        self.iframe_hash_timeout = Some(iframe_hash_timeout);
        self
    }

    pub fn set_load_frame_timeout(mut self, load_frame_timeout: u32) -> Self {
        self.load_frame_timeout = Some(load_frame_timeout);
        self
    }
}

impl From<msal::BrowserSystemOptions> for BrowserSystemOptions {
    fn from(system: msal::BrowserSystemOptions) -> Self {
        Self {
            logger_options: system.logger_options().map(Into::into),
            token_renewal_offset_seconds: system.token_renewal_offset_seconds(),
            window_hash_timeout: system.window_hash_timeout(),
            iframe_hash_timeout: system.iframe_hash_timeout(),
            load_frame_timeout: system.load_frame_timeout(),
        }
    }
}

impl From<BrowserSystemOptions> for msal::BrowserSystemOptions {
    fn from(system: BrowserSystemOptions) -> Self {
        let js_system = msal::BrowserSystemOptions::new();
        if let Some(v) = system.logger_options {
            js_system.set_logger_options(v.into())
        }
        if let Some(v) = system.token_renewal_offset_seconds {
            js_system.set_token_renewal_offset_seconds(v)
        }
        if let Some(v) = system.window_hash_timeout {
            js_system.set_window_hash_timeout(v)
        }
        if let Some(v) = system.iframe_hash_timeout {
            js_system.set_iframe_hash_timeout(v)
        }
        if let Some(v) = system.load_frame_timeout {
            js_system.set_load_frame_timeout(v)
        }
        js_system
    }
}

pub struct Configuration<'a> {
    auth: BrowserAuthOptions<'a>,
    cache: Option<CacheOptions>,
    system: Option<BrowserSystemOptions>,
}

impl<'a> Configuration<'a> {
    pub fn new(auth: BrowserAuthOptions<'a>) -> Self {
        Self {
            auth,
            cache: None,
            system: None,
        }
    }

    pub fn set_cache(mut self, cache: CacheOptions) -> Self {
        self.cache = Some(cache);
        self
    }

    pub fn set_system(mut self, system: BrowserSystemOptions) -> Self {
        self.system = Some(system);
        self
    }

    // TODO: This will panic, rather than error! So I changed from TryFrom
    // Tried using `panic::catch_unwind` but doesn't catch it?
    /// This can cause a runtime exception
    pub fn unchecked_from(js_obj: &Object) -> Self {
        js_obj
            .clone()
            .unchecked_into::<msal::Configuration>()
            .into()
    }
}

impl<'a> From<Configuration<'a>> for msal::Configuration {
    fn from(config: Configuration) -> Self {
        let js = msal::Configuration::new(&config.auth.into());
        if let Some(v) = config.cache {
            js.set_cache(v.into())
        }
        if let Some(v) = config.system {
            js.set_system(v.into())
        }
        js
    }
}

impl<'a> From<msal::Configuration> for Configuration<'a> {
    fn from(config: msal::Configuration) -> Self {
        Self {
            auth: config.auth().into(),
            cache: config.cache().map(Into::into),
            system: config.system().map(Into::into),
        }
    }
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-optional-claims#configuring-directory-extension-optional-claims
// https://tools.ietf.org/html/rfc7519#section-4.1
// https://tools.ietf.org/html/rfc7515
// https://www.iana.org/assignments/jwt/jwt.xhtml#claims
/// Covers all the claims as per the  IETF spec. If the claim doesn't match any of the standard ones
/// it will return `Custom::(claim_name, claim_value)`
/// Adds the azure specific ones too
#[derive(Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenClaim {
    typ, // Always JWT
    nonce(String),
    alg(String),
    kid(String),
    x5t(String),
    iss(String),
    sub(String),
    aud(String),
    exp(f64),
    nbf(f64),
    iat(f64),
    jti(String),
    name(String),
    given_name(String),
    family_name(String),
    middle_name(String),
    nickname(String),
    preferred_username(String),
    profile(String),
    picture(String),
    website(String),
    email(String),
    email_verified(bool),
    gender(String),
    birthdate(String),
    zoneinfo(String),
    locale(String),
    phone_number(String),
    phone_number_verified(bool),
    address(Object),
    updated_at(f64),
    cnf(Object),
    sip_from_tag(String),
    sip_date(f64),
    sip_callid(String),
    sip_cseq_num(String),
    sip_via_branch(String),
    orig(Object),
    dest(Object),
    mky(Object),
    events(Object),
    toe(f64),
    txn(String),
    rph(Object),
    sid(String),
    vot(String),
    vtm(String),
    attest(String),
    origid(String),
    act(Object),
    scope(String),
    client_id(String),
    may_act(Object),
    jcard(Object),
    at_use_nbr(f64), // Technically u32?
    div(Object),
    opt(String),
    // Azure custom
    idp(String),
    ver(String),
    oid(String),
    tid(String),
    aio(String),
    azp(String),
    azpacr(String),
    rh(String),
    scp(String),
    uti(String),
    appid(String),
    roles(Array),
    wids(Array),
    // TODO: groups:src1 // _claim_sources?
    groups(Array),
    hasgroups(bool),
    custom(String, JsValue),
}

impl TryFrom<JsValue> for TokenClaim {
    type Error = (String, JsValue);

    fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
        let kv = js_value.unchecked_into::<Array>();
        let value = kv.get(1);
        let key: String = kv.get(0).as_string().unwrap();
        let make_string = |f: &dyn Fn(String) -> Self, key, value: JsValue| match value.as_string()
        {
            Some(value) => Ok(f(value)),
            None => Err((key, value)),
        };
        let make_f64 = |f: &dyn Fn(f64) -> Self, key, value: JsValue| match value.as_f64() {
            Some(value) => Ok(f(value.to_owned())),
            None => Err((key, value)),
        };
        let make_bool = |f: &dyn Fn(bool) -> Self, key, value: JsValue| match value.as_bool() {
            Some(value) => Ok(f(value)),
            None => Err((key, value)),
        };
        let make_array = |f: &dyn Fn(Array) -> Self, key, value: JsValue| match value.dyn_into() {
            Ok(value) => Ok(f(value)),
            Err(value) => Err((key, value)),
        };
        let make_object = |f: &dyn Fn(Object) -> Self, key, value: JsValue| {
            if value.is_object() {
                Ok(f(value.unchecked_into()))
            } else {
                Err((key, value))
            }
        };

        // Returned keys are always strings
        // TODO: Could a macro be used here?
        match key.as_str() {
            "typ" => Ok(Self::typ), // Always JWT
            "nonce" => make_string(&Self::nonce, key, value),
            "alg" => make_string(&Self::alg, key, value),
            "kid" => make_string(&Self::kid, key, value),
            "x5t" => make_string(&Self::x5t, key, value),
            "iss" => make_string(&Self::iss, key, value),
            "sub" => make_string(&Self::sub, key, value),
            "aud" => make_string(&Self::aud, key, value),
            "exp" => make_f64(&Self::exp, key, value),
            "nbf" => make_f64(&Self::nbf, key, value),
            "iat" => make_f64(&Self::iat, key, value),
            "jti" => make_string(&Self::jti, key, value),
            "name" => make_string(&Self::name, key, value),
            "given_name" => make_string(&Self::given_name, key, value),
            "family_name" => make_string(&Self::family_name, key, value),
            "middle_name" => make_string(&Self::middle_name, key, value),
            "nickname" => make_string(&Self::nickname, key, value),
            "preferred_username" => make_string(&Self::preferred_username, key, value),
            "profile" => make_string(&Self::profile, key, value),
            "picture" => make_string(&Self::picture, key, value),
            "website" => make_string(&Self::website, key, value),
            "email" => make_string(&Self::email, key, value),
            "email_verified" => make_bool(&Self::email_verified, key, value),
            "gender" => make_string(&Self::gender, key, value),
            "birthdate" => make_string(&Self::birthdate, key, value),
            "zoneinfo" => make_string(&Self::zoneinfo, key, value),
            "locale" => make_string(&Self::locale, key, value),
            "phone_number" => make_string(&Self::phone_number, key, value),
            "phone_number_verified" => make_bool(&Self::phone_number_verified, key, value),
            "address" => make_object(&Self::address, key, value),
            "updated_at" => make_f64(&Self::updated_at, key, value),
            "cnf" => make_object(&Self::cnf, key, value),
            "sip_from_tag" => make_string(&Self::sip_from_tag, key, value),
            "sip_date" => make_f64(&Self::sip_date, key, value),
            "sip_callid" => make_string(&Self::sip_callid, key, value),
            "sip_cseq_num" => make_string(&Self::sip_cseq_num, key, value),
            "sip_via_branch" => make_string(&Self::sip_via_branch, key, value),
            "orig" => make_object(&Self::orig, key, value),
            "dest" => make_object(&Self::dest, key, value),
            "mky" => make_object(&Self::mky, key, value),
            "events" => make_object(&Self::events, key, value),
            "toe" => make_f64(&Self::toe, key, value),
            "txn" => make_string(&Self::txn, key, value),
            "rph" => make_object(&Self::rph, key, value),
            "sid" => make_string(&Self::sid, key, value),
            "vot" => make_string(&Self::vot, key, value),
            "vtm" => make_string(&Self::vtm, key, value),
            "attest" => make_string(&Self::attest, key, value),
            "origid" => make_string(&Self::origid, key, value),
            "act" => make_object(&Self::act, key, value),
            "scope" => make_string(&Self::scope, key, value),
            "client_id" => make_string(&Self::client_id, key, value),
            "may_act" => make_object(&Self::may_act, key, value),
            "jcard" => make_object(&Self::jcard, key, value),
            "at_use_nbr" => make_f64(&Self::at_use_nbr, key, value),
            "div" => make_object(&Self::div, key, value),
            "opt" => make_string(&Self::opt, key, value),
            // Azure
            "idp" => make_string(&Self::idp, key, value),
            "ver" => make_string(&Self::ver, key, value),
            "oid" => make_string(&Self::oid, key, value),
            "tid" => make_string(&Self::tid, key, value),
            "aio" => make_string(&Self::aio, key, value),
            "azp" => make_string(&Self::azp, key, value),
            "azpacr" => make_string(&Self::azpacr, key, value),
            "rh" => make_string(&Self::rh, key, value),
            "scp" => make_string(&Self::scp, key, value),
            "uti" => make_string(&Self::uti, key, value),
            "appid" => make_string(&Self::appid, key, value),
            "roles" => make_array(&Self::roles, key, value),
            "wids" => make_array(&Self::wids, key, value),
            "groups" => make_array(&Self::groups, key, value),
            "hasgroups" => make_bool(&Self::hasgroups, key, value),
            _ => Ok(Self::custom(key, value)),
        }
    }
}

#[derive(Clone)]
pub struct TokenClaims(pub Vec<TokenClaim>);

impl From<Object> for TokenClaims {
    fn from(js_obj: Object) -> Self {
        let mut claims = Vec::new();
        js_sys::Object::entries(&js_obj).for_each(&mut |v, _, _| {
            // If the expected type doesn't match do not return the claim
            if let Ok(v) = v.try_into() {
                claims.push(v)
            };
        });
        Self(claims)
    }
}

// TODO: Date is a Js type, should I change?
//file://./../node_modules/@azure/msal-common/dist/src/response/AuthenticationResult.d.ts
#[derive(Clone)]
pub struct AuthenticationResult {
    unique_id: String,
    tenant_id: String,
    scopes: Vec<String>,
    account: AccountInfo,
    id_token: String,
    id_token_claims: TokenClaims,
    access_token: String,
    from_cache: bool,
    expires_on: Date,
    ext_expires_on: Option<Date>,
    state: Option<String>,
    family_id: Option<String>,
}

impl AuthenticationResult {
    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn scopes(&self) -> &Vec<String> {
        &self.scopes
    }

    pub fn account(&self) -> &AccountInfo {
        &self.account
    }

    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    pub fn id_token_claims(&self) -> &TokenClaims {
        &self.id_token_claims
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn from_cache(&self) -> &bool {
        &self.from_cache
    }

    pub fn expires_on(&self) -> &Date {
        &self.expires_on
    }

    pub fn ext_expires_on(&self) -> Option<&Date> {
        self.ext_expires_on.as_ref()
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    pub fn family_id(&self) -> Option<&str> {
        self.family_id.as_deref()
    }
}

impl From<msal::AuthenticationResult> for AuthenticationResult {
    fn from(auth_result: msal::AuthenticationResult) -> Self {
        Self {
            unique_id: auth_result.unique_id(),
            tenant_id: auth_result.tenant_id(),
            scopes: JsArrayString::from(auth_result.scopes()).0,
            account: auth_result.account().into(),
            id_token: auth_result.id_token(),
            id_token_claims: auth_result.id_token_claims().into(),
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

pub trait PublicClientApplication: msal::Msal {
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
            .get_account_by_username(username.into())
            .map(Into::into)
    }

    fn get_account_by_home_id(&self, home_id: &str) -> Option<AccountInfo> {
        self.auth()
            .get_account_by_home_id(home_id.into())
            .map(Into::into)
    }

    fn logout<'a>(&self, request: Option<EndSessionRequest<'a>>) {
        self.auth().logout(request.unwrap_or_default().into())
    }
}

// Can't put these on the trait since `async` is not allowed in traits
// https://rust-lang.github.io/async-book/07_workarounds/06_async_in_traits.html
// https://github.com/dtolnay/async-trait

/// Silent login https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/login-user.md#silent-login-with-ssosilent
/// needs a login_hint, sid or account object on the request
async fn sso_silent<'a>(
    client_app: &msal::PublicClientApplication,
    request: &'a AuthorizationUrlRequest<'a>,
) -> Result<AuthenticationResult, JsValue> {
    client_app.sso_silent(request.into()).await.map(Into::into)
}

// Called by both popup and redirect
// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/acquire-token.md
// Call this first, then if it fails will will need to call the interactive methods
async fn acquire_token_silent<'a>(
    client_app: &msal::PublicClientApplication,
    request: &'a SilentRequest<'a>,
) -> Result<AuthenticationResult, JsValue> {
    client_app
        .acquire_token_silent(request.into())
        .await
        .map(Into::into)
}

#[derive(Clone)]
pub struct AccountInfo {
    home_account_id: String,
    environment: String,
    tenant_id: String,
    username: String,
}

impl AccountInfo {
    pub fn home_account_id(&self) -> &str {
        &self.home_account_id
    }

    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    fn from_array(array: Array) -> Vec<Self> {
        array
            .iter()
            .map(|v| v.unchecked_into::<msal::AccountInfo>().into())
            .collect()
    }
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

impl<'a> From<&'a AccountInfo> for msal::AccountInfo {
    fn from(account_info: &'a AccountInfo) -> Self {
        msal::AccountInfo::new(
            &account_info.home_account_id,
            &account_info.environment,
            &account_info.tenant_id,
            &account_info.username,
        )
    }
}

pub mod prelude {
    pub use crate::popup::PopupApp;
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
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::*;
    use web_sys::console;

    pub const CLIENT_ID: &str = "MY_CLIENT_ID";
    pub const REDIRECT_URI: &str = "MY_REDIRECT_URI";
    pub const HOME_ACCOUNT_ID: &str = "home_account_id";
    pub const ENVIRONMENT: &str = "environment";
    pub const TENANT_ID: &str = "tenant_id";
    pub const USERNAME: &str = "username";
    pub const SCOPE: &str = "scope";
    pub const AUTHORITY: &str = "authority";
    pub const CORRELATION_ID: &str = "correlation_id";
    pub const POST_LOGOUT_URI: &str = "POST_LOGOUT_URI";
    pub const CLOUD_DISCOVERY_METADATA: &str = "CLOUD_DISCOVERY_METADATA";
    pub const KNOWN_AUTHORITIES: [&str; 1] = ["KNOWN_AUTHORITIES"];
    pub const NAVIGATE_TO_LOGIN_REQUEST_URL: bool = true;
    pub const POST_LOGOUT_REDIRECT_URI: &str = "POST_LOGOUT_REDIRECT_URI";

    #[wasm_bindgen(module = "/msal-object-examples.js")]
    extern "C" {
        static authResponse: Object;
        static auth: Object;
        static cache: Object;
        static system: Object;
        static msalConfig: Object;
        static accessToken: Object;
        static idToken: Object;
        static completeToken: Object;
    }

    fn home_account_id<'a>(i: usize) -> Cow<'a, str> {
        format!("home_account_id_{}", i).into()
    }
    fn environment<'a>(i: usize) -> Cow<'a, str> {
        format!("environment_{}", i).into()
    }
    fn tenant_id<'a>(i: usize) -> Cow<'a, str> {
        format!("tenant_id_{}", i).into()
    }
    fn username<'a>(i: usize) -> Cow<'a, str> {
        format!("username_{}", i).into()
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
            &home_account_id(i),
            &environment(i),
            &tenant_id(i),
            &username(i),
        )
    }

    #[wasm_bindgen_test]
    fn mirror_account_info() {
        let js_ac: msal::AccountInfo = (&account()).into();
        assert_eq!(js_ac.home_account_id(), account().home_account_id);
        assert_eq!(js_ac.environment(), account().environment);
        assert_eq!(js_ac.tenant_id(), account().tenant_id);
        assert_eq!(js_ac.username(), account().username);

        js_cast_checker::<msal::AccountInfo>(js_ac.into());
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

    #[wasm_bindgen_test]
    fn parse_js_authentication_result() {
        let _: AuthenticationResult = authResponse
            .clone()
            .unchecked_into::<msal::AuthenticationResult>()
            .into();
    }

    #[wasm_bindgen_test]
    fn parse_js_browser_auth_options() {
        let _: BrowserAuthOptions = auth
            .clone()
            .unchecked_into::<msal::BrowserAuthOptions>()
            .into();
        // TODO: Check the values
    }

    #[wasm_bindgen_test]
    fn parse_js_cache_options() {
        let _: CacheOptions = cache.clone().unchecked_into::<msal::CacheOptions>().into();
        // TODO: Check the values
    }

    #[wasm_bindgen_test]
    fn parse_js_system() {
        let _: BrowserSystemOptions = cache
            .clone()
            .unchecked_into::<msal::BrowserSystemOptions>()
            .into();
        // TODO: Check the values
    }

    #[wasm_bindgen_test]
    fn parse_js_configuration() {
        let config: Configuration = Configuration::unchecked_from(&msalConfig);
        let js_config: msal::Configuration = config.into();
        console::log_1(&"JS Configuration:".into());
        console::log_1(&js_config.clone().into());

        js_cast_checker::<msal::Configuration>(js_config.into());
    }

    #[wasm_bindgen_test]
    fn build_browser_auth_options() {
        let b_auth = BrowserAuthOptions::new(CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_cloud_discovery_metadata(CLOUD_DISCOVERY_METADATA)
            .set_known_authorities(&KNOWN_AUTHORITIES[..])
            .set_navigate_to_login_request_url(NAVIGATE_TO_LOGIN_REQUEST_URL)
            .set_post_logout_redirect_uri(POST_LOGOUT_REDIRECT_URI)
            .set_redirect_uri(REDIRECT_URI);

        let b_cache = CacheOptions::new()
            .set_cache_location(CacheLocation::Session)
            .set_store_auth_state_in_cookie(true);

        fn logger_callback(_: LogLevel, _: String, _: bool) {};

        let logger_options = LoggerOptions::new()
            .set_log_level(LogLevel::Info)
            .set_pii_logging_enabled(true)
            .set_logger_callback(&logger_callback);

        let b_system = BrowserSystemOptions::new()
            .set_token_renewal_offset_seconds(66)
            .set_iframe_hash_timeout(66)
            .set_load_frame_timeout(66)
            .set_window_hash_timeout(66)
            .set_logger_options(logger_options);

        let config = Configuration::new(b_auth)
            .set_cache(b_cache)
            .set_system(b_system);

        let js_config: msal::Configuration = config.into();

        console::log_1(&"WASM Configuration:".into());
        console::log_1(&js_config.clone().into());

        js_cast_checker::<msal::Configuration>(js_config.into());
    }

    #[wasm_bindgen_test]
    fn parse_access_token() {
        let _: TokenClaims = accessToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn parse_id_token() {
        let _: TokenClaims = idToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn parse_claims() {
        let id_claims: TokenClaims = idToken.clone().into();
        let access_claims: TokenClaims = accessToken.clone().into();
        let claim = id_claims
            .0
            .iter()
            .find_map(|v| {
                if let TokenClaim::alg(c) = v {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(claim, "RS256");

        let no_custom = |claims: TokenClaims| {
            claims.0.into_iter().find_map(|v| {
                if let TokenClaim::custom(c, v) = v {
                    Some((c, v))
                } else {
                    None
                }
            })
        };

        let all: TokenClaims = completeToken.clone().into();

        // Check have found all azure claims, the source may not have them all though!
        assert!(no_custom(id_claims).is_none());
        assert!(no_custom(access_claims).is_none());
        assert!(no_custom(all).is_none());
    }
    // TODO: Add a suite of integration tests to ensure the API is stable?
}
