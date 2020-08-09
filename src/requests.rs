use crate::{msal, AccountInfo};
use msal::JsArrayString;
use std::{borrow::Cow, collections::HashMap, fmt::Display};

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
pub struct BaseAuthRequest<'a> {
    scopes: Vec<Cow<'a, str>>,
    authority: Option<Cow<'a, str>>,
    correlation_id: Option<Cow<'a, str>>,
}

impl<'a> BaseAuthRequest<'a> {
    pub fn set_authority<T>(mut self, authority: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.authority = Some(authority.into());
        self
    }

    pub fn set_correlation_id<T>(mut self, correlation_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

// impl From<Vec<String>> for BaseAuthRequest {
//     fn from(scopes: Vec<String>) -> Self {
//         Self {
//             scopes,
//             authority: None,
//             correlation_id: None,
//         }
//     }
// }

// impl From<Vec<&str>> for BaseAuthRequest {
//     fn from(scopes: Vec<&str>) -> Self {
//         scopes
//             .into_iter()
//             .map(String::from)
//             .collect::<Vec<String>>()
//             .into()
//     }
// }

impl<'a, T> From<T> for BaseAuthRequest<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(scope: T) -> Self {
        Self {
            scopes: vec![scope.into()],
            authority: None,
            correlation_id: None,
        }
    }
}
#[allow(dead_code)]
pub struct AuthorizationUrlRequest<'a> {
    base_request: Cow<'a, BaseAuthRequest<'a>>,
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

impl<'a> AuthorizationUrlRequest<'a> {
    /// Can be retrieved from the account object username property or the upn claim in the ID token
    pub fn set_login_hint(mut self, login_hint: &str) -> Self {
        match self.login_hint.as_mut() {
            Some(s) => s.replace_range(.., login_hint),
            None => self.login_hint = Some(String::from(login_hint)),
        }
        self
    }
}

impl<'a> From<AuthorizationUrlRequest<'a>> for msal::AuthorizationUrlRequest {
    // TODO: Add in all fields
    fn from(request: AuthorizationUrlRequest) -> Self {
        let auth_req = msal::AuthorizationUrlRequest::new(
            &JsArrayString::from(request.base_request.scopes.clone()).into(),
        );

        request
            .base_request
            .authority
            .iter()
            .for_each(|s| auth_req.set_authority(s));

        request
            .base_request
            .correlation_id
            .iter()
            .for_each(|s| auth_req.set_correlation_id(s));

        request
            .login_hint
            .into_iter()
            .for_each(|s| auth_req.set_login_hint(s.as_str()));

        auth_req
    }
}

impl<'a> From<Vec<&'a str>> for AuthorizationUrlRequest<'a> {
    fn from(scopes: Vec<&str>) -> Self {
        scopes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .into()
    }
}

impl<'a> From<Vec<String>> for AuthorizationUrlRequest<'a> {
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

impl<'a> From<&'a BaseAuthRequest<'a>> for AuthorizationUrlRequest<'a> {
    fn from(base_request: &'a BaseAuthRequest<'a>) -> Self {
        Self {
            base_request: Cow::Borrowed(base_request),
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
pub struct SilentRequest<'a> {
    base_request: &'a BaseAuthRequest<'a>,
    account: &'a AccountInfo<'a>,
    force_refresh: Option<bool>,
    redirect_uri: Option<Cow<'a, str>>,
}

impl<'a> SilentRequest<'a> {
    pub fn from_account_info(
        base_request: &'a BaseAuthRequest<'a>,
        account_info: &'a AccountInfo<'a>,
    ) -> Self {
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

    pub fn set_redirect_uri<T>(mut self, redirect_uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }
}

impl<'a> From<&'a SilentRequest<'a>> for msal::SilentRequest {
    fn from(request: &'a SilentRequest) -> Self {
        let r = msal::SilentRequest::new(
            &JsArrayString::from(request.base_request.scopes.clone()).into(),
            request.account.into(),
        );
        request.base_request.authority.iter().for_each(|v| r.set_authority(v));
        request
            .base_request
            .correlation_id.iter().for_each(|v| r.set_correlation_id(v));
        request
            .force_refresh
            .into_iter()
            .for_each(|v| r.set_force_refresh(v));
        request
            .redirect_uri
            .iter()
            .for_each(|v| r.set_redirect_uri(v));
        r
    }
}

#[derive(Default, Clone)]
pub struct EndSessionRequest<'a> {
    account: Option<Cow<'a, AccountInfo<'a>>>,
    post_logout_redirect_uri: Option<Cow<'a, str>>,
    authority: Option<Cow<'a, str>>,
    correlation_id: Option<Cow<'a, str>>,
}

impl<'a> EndSessionRequest<'a> {
    pub fn set_account(mut self, account: &'a AccountInfo<'a>) -> Self {
        self.account = Some(Cow::Borrowed(account));
        self
    }

    pub fn set_post_logout_redirect_uri<T>(mut self, post_logout_redirect_uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.post_logout_redirect_uri = Some(post_logout_redirect_uri.into());
        self
    }

    pub fn set_authority<T>(mut self, authority: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.authority = Some(authority.into());
        self
    }

    pub fn set_correlation_id<T>(mut self, correlation_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

impl<'a> From<EndSessionRequest<'a>> for msal::EndSessionRequest {
    fn from(request: EndSessionRequest) -> Self {
        let r = msal::EndSessionRequest::new();
        request.account.iter().for_each(|v| {
            r.set_account(v.as_ref().into());
        });
        request
            .post_logout_redirect_uri
            .into_iter()
            .for_each(|v| r.set_post_logout_redirect_uri(&v));
        request
            .authority
            .into_iter()
            .for_each(|v| r.set_authority(&v));
        request
            .correlation_id
            .into_iter()
            .for_each(|v| r.set_correlation_id(&v));
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

    fn base_req() -> BaseAuthRequest<'static> {
        BaseAuthRequest::from(SCOPE)
            .set_authority(AUTHORITY)
            .set_correlation_id(CORRELATION_ID)
    }

    #[wasm_bindgen_test]
    fn mirror_silent_request() {
        // let bas_req =
        let req = SilentRequest::from_account_info(&base_req(), &account())
            .set_force_refresh(FORCE_REFRESH)
            .set_redirect_uri(REDIRECT_URI);

        let js_req: msal::SilentRequest = (&req).into();

        assert_eq!(
            req.base_request.scopes,
            JsArrayString::from(js_req.scopes()).0
        );
        assert_eq!(
            req.base_request.correlation_id.map(Cow::into_owned),
            js_req.correlation_id()
        );
        assert_eq!(
            req.base_request.authority.map(Cow::into_owned),
            js_req.authority()
        );
        assert_eq!(
            req.account.home_account_id,
            js_req.account().home_account_id()
        );
        assert_eq!(req.account.environment, js_req.account().environment());
        assert_eq!(req.account.tenant_id, js_req.account().tenant_id());
        assert_eq!(req.account.username, js_req.account().username());
        assert_eq!(req.force_refresh, js_req.force_refresh());
        assert_eq!(req.redirect_uri.map(Cow::into_owned), js_req.redirect_uri());

        js_cast_checker::<msal::SilentRequest>(js_req.into());
    }

    #[wasm_bindgen_test]
    fn mirror_end_session_request() {
        let req = EndSessionRequest::default()
            .set_account(&account())
            .set_authority(AUTHORITY)
            .set_correlation_id(CORRELATION_ID)
            .set_post_logout_redirect_uri(POST_LOGOUT_URI);

        let js_req: msal::EndSessionRequest = req.clone().into();
        assert_eq!(
            req.correlation_id.map(Cow::into_owned),
            js_req.correlation_id()
        );
        assert_eq!(
            req.post_logout_redirect_uri.map(Cow::into_owned),
            js_req.post_logout_redirect_uri()
        );
        assert_eq!(req.authority.map(Cow::into_owned), js_req.authority());
        assert_eq!(
            req.account.as_ref().unwrap().home_account_id,
            js_req.account().unwrap().home_account_id()
        );
        assert_eq!(
            req.account.as_ref().unwrap().environment,
            js_req.account().unwrap().environment()
        );
        assert_eq!(
            req.account.as_ref().unwrap().tenant_id,
            js_req.account().unwrap().tenant_id()
        );
        assert_eq!(
            req.account.as_ref().unwrap().username,
            js_req.account().unwrap().username()
        );

        js_cast_checker::<msal::EndSessionRequest>(js_req.into());
    }
}
