use crate::{msal, set_option_string, AccountInfo};
use msal::{JsArrayString, JsMirror};
use std::{collections::HashMap, fmt::Display};

pub enum ResponseMode {
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

#[derive(Clone)]
pub struct BaseAuthRequest {
    scopes: Vec<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl BaseAuthRequest {
    pub fn set_authority(mut self, authority: &str) -> Self {
        match self.authority.as_mut() {
            Some(a) => a.replace_range(.., authority),
            None => self.authority = Some(String::from(authority)),
        }
        self
    }

    pub fn set_correlation_id(mut self, correlation_id: &str) -> Self {
        match self.correlation_id.as_mut() {
            Some(a) => a.replace_range(.., correlation_id),
            None => self.correlation_id = Some(String::from(correlation_id)),
        }
        self
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

impl From<&str> for BaseAuthRequest {
    fn from(scope: &str) -> Self {
        vec![scope.to_string()].into()
    }
}
#[allow(dead_code)]
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
    /// Can be retrieved from the account object username property or the upn claim in the ID token
    pub fn set_login_hint(mut self, login_hint: &str) -> Self {
        match self.login_hint.as_mut() {
            Some(s) => s.replace_range(.., login_hint),
            None => self.login_hint = Some(String::from(login_hint)),
        }
        self
    }
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
        request
            .login_hint
            .into_iter()
            .for_each(|s| auth_req.set_login_hint(s));
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
#[allow(dead_code)]
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
    // TODO: Add in all fields
    fn from(request: RedirectRequest) -> Self {
        let auth_req = msal::RedirectRequest::new(
            &JsArrayString::from(request.auth_url_req.base_request.scopes.clone()).into(),
        );
        auth_req
    }
}

#[derive(Clone)]
pub struct SilentRequest {
    base_request: BaseAuthRequest,
    account: AccountInfo,
    force_refresh: Option<bool>,
    redirect_uri: Option<String>,
}

impl SilentRequest {
    pub fn from_account_info(base_request: BaseAuthRequest, account_info: AccountInfo) -> Self {
        Self {
            base_request,
            account: account_info,
            force_refresh: None,
            redirect_uri: None,
        }
    }

    pub fn set_force_refresh(mut self, force_refresh: bool) -> Self {
        self.force_refresh = Some(force_refresh);
        self
    }

    pub fn set_redirect_uri(mut self, redirect_uri: &str) -> Self {
        match self.redirect_uri.as_mut() {
            Some(s) => s.replace_range(.., redirect_uri),
            None => self.redirect_uri = Some(redirect_uri.to_string()),
        }
        self
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

#[derive(Default, Clone)]
pub struct EndSessionRequest {
    account: Option<AccountInfo>,
    post_logout_redirect_uri: Option<String>,
    authority: Option<String>,
    correlation_id: Option<String>,
}

impl EndSessionRequest {
    pub fn set_account(mut self, account: AccountInfo) -> Self {
        self.account = Some(account);
        self
    }

    pub fn set_post_logout_redirect_uri(mut self, post_logout_redirect_uri: &str) -> Self {
        set_option_string(&mut self.post_logout_redirect_uri, post_logout_redirect_uri);
        self
    }

    pub fn set_authority(mut self, authority: &str) -> Self {
        set_option_string(&mut self.authority, authority);
        self
    }

    pub fn set_correlation_id(mut self, correlation_id: &str) -> Self {
        set_option_string(&mut self.correlation_id, correlation_id);
        self
    }
}

impl JsMirror for EndSessionRequest {
    type JsTarget = msal::EndSessionRequest;
}

impl From<EndSessionRequest> for msal::EndSessionRequest {
    fn from(request: EndSessionRequest) -> Self {
        let r = msal::EndSessionRequest::new();
        request.account.into_iter().for_each(|v| {
            r.set_account(v.into());
        });
        request
            .post_logout_redirect_uri
            .into_iter()
            .for_each(|v| r.set_post_logout_redirect_uri(v));
        request
            .authority
            .into_iter()
            .for_each(|v| r.set_authority(v));
        request
            .correlation_id
            .into_iter()
            .for_each(|v| r.set_correlation_id(v));
        r
    }
}

#[cfg(test)]
mod test_request {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use crate::tests::*;
    use wasm_bindgen_test::*;

    const FORCE_REFRESH: bool = true;
    const REDIRECT_URI: &str = "redirect_uri";

    #[wasm_bindgen_test]
    fn mirror_auth_url_request() {
        // TODO: Write tests
    }

    #[wasm_bindgen_test]
    fn mirror_redirect_request() {
        // TODO: Write tests
    }

    fn base_req() -> BaseAuthRequest {
        BaseAuthRequest::from(SCOPE)
            .set_authority(AUTHORITY)
            .set_correlation_id(CORRELATION_ID)
    }

    #[wasm_bindgen_test]
    fn mirror_silent_request() {
        let req = SilentRequest::from_account_info(base_req(), account())
            .set_force_refresh(FORCE_REFRESH)
            .set_redirect_uri(REDIRECT_URI);

        let js_req: msal::SilentRequest = req.clone().into();

        assert_eq!(
            req.base_request.scopes,
            JsArrayString::from(js_req.scopes()).0
        );
        assert_eq!(req.base_request.correlation_id, js_req.correlation_id());
        assert_eq!(req.base_request.authority, js_req.authority());
        assert_eq!(
            req.account.home_account_id,
            js_req.account().home_account_id()
        );
        assert_eq!(req.account.environment, js_req.account().environment());
        assert_eq!(req.account.tenant_id, js_req.account().tenant_id());
        assert_eq!(req.account.username, js_req.account().username());
        assert_eq!(req.force_refresh, js_req.force_refresh());
        assert_eq!(req.redirect_uri, js_req.redirect_uri());

        js_cast_checker::<msal::SilentRequest>(js_req.into());
    }

    #[wasm_bindgen_test]
    fn mirror_end_session_request() {
        let req = EndSessionRequest::default()
            .set_account(account())
            .set_authority(AUTHORITY)
            .set_correlation_id(CORRELATION_ID)
            .set_post_logout_redirect_uri(POST_LOGOUT_URI);

        let js_req: msal::EndSessionRequest = req.clone().into();
        assert_eq!(req.correlation_id, js_req.correlation_id());
        assert_eq!(
            req.post_logout_redirect_uri,
            js_req.post_logout_redirect_uri()
        );
        assert_eq!(req.authority, js_req.authority());
        assert_eq!(
            req.account.as_ref().unwrap().home_account_id,
            js_req.account().unwrap().home_account_id()
        );
        assert_eq!(req.account.as_ref().unwrap().environment, js_req.account().unwrap().environment());
        assert_eq!(req.account.as_ref().unwrap().tenant_id, js_req.account().unwrap().tenant_id());
        assert_eq!(req.account.as_ref().unwrap().username, js_req.account().unwrap().username());

        js_cast_checker::<msal::EndSessionRequest>(js_req.into());
    }
}
