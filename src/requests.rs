use crate::{msal, AccountInfo};
use msal::{JsArrayString, JsMirror};
use std::{collections::HashMap, fmt::Display};

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
struct BaseAuthRequest {
    scopes: Vec<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl BaseAuthRequest {
    pub fn set_authority(&mut self, authority: &str) {
        match self.authority.as_mut() {
            Some(a) => a.replace_range(.., authority),
            None => self.authority = Some(String::from(authority)),
        }
    }

    pub fn set_correlation_id(&mut self, correlation_id: &str) {
        match self.correlation_id.as_mut() {
            Some(a) => a.replace_range(.., correlation_id),
            None => self.correlation_id = Some(String::from(correlation_id)),
        }
    }

    fn clone_authority<F>(&self, cloner: F)
    where
        F: Fn(String),
    {
        self.authority.iter().for_each(|s| cloner(s.clone()));
    }

    fn clone_correlation_id<F>(&self, cloner: F)
    where
        F: Fn(String),
    {
        self.correlation_id.iter().for_each(|s| cloner(s.clone()));
    }
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

impl JsMirror for AuthorizationUrlRequest {
    type JsTarget = msal::AuthorizationUrlRequest;
}

impl From<AuthorizationUrlRequest> for msal::AuthorizationUrlRequest {
    // TODO: Add in all fields
    fn from(request: AuthorizationUrlRequest) -> Self {
        let auth_req = msal::AuthorizationUrlRequest::new(
            &JsArrayString::from(request.base_request.scopes.clone()).into(),
        );
        request
            .base_request
            .clone_authority(|s| auth_req.set_authority(s));
        request
            .base_request
            .clone_correlation_id(|s| auth_req.set_correlation_id(s));
        auth_req
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

#[cfg(feature = "redirect")]
pub struct RedirectRequest {
    auth_url_req: AuthorizationUrlRequest,
    redirect_start_page: Option<String>,
}

#[cfg(feature = "redirect")]
impl JsMirror for RedirectRequest {
    type JsTarget = msal::RedirectRequest;
}

#[cfg(feature = "redirect")]
impl RedirectRequest {
    pub fn new(scopes: Vec<String>) -> Self {
        Self {
            auth_url_req: AuthorizationUrlRequest::from(scopes),
            redirect_start_page: None,
        }
    }
}

#[cfg(feature = "redirect")]
impl From<Vec<&str>> for RedirectRequest {
    fn from(scopes: Vec<&str>) -> Self {
        scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .into()
    }
}

#[cfg(feature = "redirect")]
impl From<Vec<String>> for RedirectRequest {
    fn from(scopes: Vec<String>) -> Self {
        Self {
            auth_url_req: scopes.into(),
            redirect_start_page: None,
        }
    }
}

#[cfg(feature = "redirect")]
impl From<RedirectRequest> for msal::RedirectRequest {
    //TODO: Add in all the values
    fn from(request: RedirectRequest) -> Self {
        let auth_req = msal::RedirectRequest::new(
            &JsArrayString::from(request.auth_url_req.base_request.scopes.clone()).into(),
        );
        auth_req
    }
}

pub struct SilentRequest {
    base_request: BaseAuthRequest,
    account: AccountInfo,
    force_refresh: Option<bool>,
    redirect_uri: Option<String>,
}

impl SilentRequest {
    fn from_account_info(base_request: BaseAuthRequest, account_info: AccountInfo) -> Self {
        Self {
            base_request,
            account: account_info,
            force_refresh: None,
            redirect_uri: None,
        }
    }
}

impl JsMirror for SilentRequest {
    type JsTarget = msal::SilentRequest;
}

impl From<SilentRequest> for msal::SilentRequest {
    fn from(request: SilentRequest) -> Self {
        let r = msal::SilentRequest::new(
            &JsArrayString::from(request.base_request.scopes.clone()).into(),
            request.account.into(),
        );
        request.base_request.clone_authority(|v| r.set_authority(v));
        request
            .base_request
            .clone_correlation_id(|v| r.set_correlation_id(v));
        request
            .force_refresh
            .into_iter()
            .for_each(|v| r.set_force_refresh(v));
        request
            .redirect_uri
            .into_iter()
            .for_each(|v| r.set_redirect_uri(v));
        r
    }
}

#[derive(Default)]
pub struct EndSessionRequest {
    account: Option<String>,
    post_logout_redirect_uri: Option<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl JsMirror for EndSessionRequest {
    type JsTarget = msal::EndSessionRequest;
}

impl From<EndSessionRequest> for msal::EndSessionRequest {
    fn from(request: EndSessionRequest) -> Self {
        let r = msal::EndSessionRequest::new();
        request.account.into_iter().for_each(|v| {
            r.set_account(v);
        });
        request
            .post_logout_redirect_uri
            .into_iter()
            .for_each(|v| r.set_account(v));
        request.authority.into_iter().for_each(|v| r.set_account(v));
        request
            .correlation_id
            .into_iter()
            .for_each(|v| r.set_account(v));
        r
    }
}
