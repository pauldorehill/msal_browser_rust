use crate::{msal, AccountInfo};
use msal::{JsArrayString, JsHashMapStrStr};
use std::{borrow::Cow, collections::HashMap};

pub enum ResponseMode {
    Query,
    Fragment,
    FormPost,
}

impl ResponseMode {
    fn as_str(&self) -> &str {
        match &self {
            ResponseMode::Query => "query",
            ResponseMode::Fragment => "fragment",
            ResponseMode::FormPost => "form_post",
        }
    }
}

/// No scopes required since all the request constructors require Scopes
struct IterBaseAuthRequest<'a, T> {
    base_auth_request: &'a BaseAuthRequest<'a>,
    destination: &'a T,
    authority: &'a dyn Fn(&T, &Cow<'a, str>),
    correlation_id: &'a dyn Fn(&T, &Cow<'a, str>),
}

impl<'a, T> IterBaseAuthRequest<'a, T> {
    fn iter_all(self) {
        if let Some(v) = &self.base_auth_request.authority {
            (self.authority)(self.destination, v)
        }
        if let Some(v) = &self.base_auth_request.correlation_id {
            (self.correlation_id)(self.destination, v)
        }
    }
}

#[derive(Clone)]
pub struct BaseAuthRequest<'a> {
    scopes: Vec<Cow<'a, str>>, // Leave as Vec in case want to update as using?
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

impl<'a, T> From<&'a [T]> for BaseAuthRequest<'a>
where
    T: Into<Cow<'a, str>> + std::clone::Clone,
{
    fn from(scopes: &'a [T]) -> Self {
        let scopes: Vec<Cow<'a, str>> = scopes.iter().cloned().map(|v| v.into()).collect();
        Self {
            scopes,
            authority: None,
            correlation_id: None,
        }
    }
}

pub struct AuthorizationUrlRequest<'a> {
    base_request: BaseAuthRequest<'a>, //TODO: Can I make this a reference?
    redirect_uri: Option<Cow<'a, str>>,
    extra_scopes_to_consent: Option<Vec<Cow<'a, str>>>,
    response_mode: Option<ResponseMode>,
    code_challenge: Option<Cow<'a, str>>,
    code_challenge_method: Option<Cow<'a, str>>,
    state: Option<Cow<'a, str>>,
    prompt: Option<Cow<'a, str>>, //TODO: this is an enum
    login_hint: Option<Cow<'a, str>>,
    domain_hint: Option<Cow<'a, str>>,
    extra_query_parameters: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>, //TODO: Is this ok?
    claims: Option<Cow<'a, str>>,
    nonce: Option<Cow<'a, str>>,
}

impl<'a> AuthorizationUrlRequest<'a> {
    pub fn set_login_hint<T>(mut self, login_hint: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.login_hint = Some(login_hint.into());
        self
    }
}

/// This is here since it is used as the basis for other requests so I can avoid
/// code duplication
struct IterAuthorizationUrlRequest<'a, T> {
    auth_url_request: &'a AuthorizationUrlRequest<'a>,
    destination: &'a T,
    base_request: IterBaseAuthRequest<'a, T>,
    redirect_uri: &'a dyn Fn(&T, &Cow<'a, str>),
    extra_scopes_to_consent: &'a dyn Fn(&T, &Vec<Cow<'a, str>>),
    response_mode: &'a dyn Fn(&T, &ResponseMode),
    code_challenge: &'a dyn Fn(&T, &Cow<'a, str>),
    code_challenge_method: &'a dyn Fn(&T, &Cow<'a, str>),
    state: &'a dyn Fn(&T, &Cow<'a, str>),
    prompt: &'a dyn Fn(&T, &Cow<'a, str>),
    login_hint: &'a dyn Fn(&T, &Cow<'a, str>),
    domain_hint: &'a dyn Fn(&T, &Cow<'a, str>),
    extra_query_parameters: &'a dyn Fn(&T, &HashMap<Cow<'a, str>, Cow<'a, str>>),
    claims: &'a dyn Fn(&T, &Cow<'a, str>),
    nonce: &'a dyn Fn(&T, &Cow<'a, str>),
}

impl<'a, T> IterAuthorizationUrlRequest<'a, T> {
    fn iter_all(self) {
        self.base_request.iter_all();
        if let Some(v) = &self.auth_url_request.redirect_uri {
            (self.redirect_uri)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.extra_scopes_to_consent {
            (self.extra_scopes_to_consent)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.response_mode {
            (self.response_mode)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.code_challenge {
            (self.code_challenge)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.code_challenge_method {
            (self.code_challenge_method)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.state {
            (self.state)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.prompt {
            (self.prompt)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.login_hint {
            (self.login_hint)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.domain_hint {
            (self.domain_hint)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.extra_query_parameters {
            (self.extra_query_parameters)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.claims {
            (self.claims)(self.destination, v)
        }
        if let Some(v) = &self.auth_url_request.nonce {
            (self.nonce)(self.destination, v)
        }
    }
}

impl<'a> From<&'a AuthorizationUrlRequest<'a>> for msal::AuthorizationUrlRequest {
    fn from(request: &'a AuthorizationUrlRequest<'a>) -> Self {
        let js = msal::AuthorizationUrlRequest::new(
            &JsArrayString::from(&request.base_request.scopes).into(),
        );

        IterAuthorizationUrlRequest {
            auth_url_request: request,
            destination: &js,
            base_request: IterBaseAuthRequest {
                base_auth_request: &request.base_request,
                destination: &js,
                authority: &|js, v| js.set_authority(v),
                correlation_id: &|js, v| js.set_correlation_id(v),
            },
            redirect_uri: &|js, v| js.set_redirect_uri(v),
            extra_scopes_to_consent: &|js, v| {
                js.set_extra_scopes_to_consent(JsArrayString::from(v).into())
            },
            response_mode: &|js, v| js.set_response_mode(v.as_str()),
            code_challenge: &|js, v| js.set_code_challenge(v),
            code_challenge_method: &|js, v| js.set_code_challenge_method(v),
            state: &|js, v| js.set_state(v),
            prompt: &|js, v| js.set_prompt(v),
            login_hint: &|js, v| js.set_login_hint(v),
            domain_hint: &|js, v| js.set_domain_hint(v),
            extra_query_parameters: &|js, v| {
                js.set_extra_query_parameters(JsHashMapStrStr::from(v).into())
            },
            claims: &|js, v| js.set_claims(v),
            nonce: &|js, v| js.set_nonce(v),
        }
        .iter_all();
        js
    }
}

impl<'a, T> From<&'a [T]> for AuthorizationUrlRequest<'a>
where
    T: Into<Cow<'a, str>> + std::clone::Clone,
{
    fn from(scopes: &'a [T]) -> Self {
        Self {
            base_request: scopes.into(),
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
            base_request: base_request.clone(),
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
pub struct RedirectRequest<'a> {
    auth_url_req: AuthorizationUrlRequest<'a>,
    redirect_start_page: Option<Cow<'a, str>>,
}

#[cfg(feature = "redirect")]
impl<'a, T> From<&'a [T]> for RedirectRequest<'a>
where
    T: Into<Cow<'a, str>> + std::clone::Clone,
{
    fn from(scopes: &'a [T]) -> Self {
        Self {
            auth_url_req: scopes.into(),
            redirect_start_page: None,
        }
    }
}

#[cfg(feature = "redirect")]
impl<'a> From<&'a RedirectRequest<'a>> for msal::RedirectRequest {
    fn from(request: &'a RedirectRequest<'a>) -> Self {
        let js = msal::RedirectRequest::new(
            &JsArrayString::from(&request.auth_url_req.base_request.scopes).into(),
        );

        IterAuthorizationUrlRequest {
            auth_url_request: &request.auth_url_req,
            destination: &js,
            base_request: IterBaseAuthRequest {
                base_auth_request: &request.auth_url_req.base_request,
                destination: &js,
                authority: &|js, v| js.set_authority(v),
                correlation_id: &|js, v| js.set_correlation_id(v),
            },
            redirect_uri: &|js, v| js.set_redirect_uri(v),
            extra_scopes_to_consent: &|js, v| {
                js.set_extra_scopes_to_consent(JsArrayString::from(v).into())
            },
            response_mode: &|js, v| js.set_response_mode(v.as_str()),
            code_challenge: &|js, v| js.set_code_challenge(v),
            code_challenge_method: &|js, v| js.set_code_challenge_method(v),
            state: &|js, v| js.set_state(v),
            prompt: &|js, v| js.set_prompt(v),
            login_hint: &|js, v| js.set_login_hint(v),
            domain_hint: &|js, v| js.set_domain_hint(v),
            extra_query_parameters: &|js, v| {
                js.set_extra_query_parameters(JsHashMapStrStr::from(v).into())
            },
            claims: &|js, v| js.set_claims(v),
            nonce: &|js, v| js.set_nonce(v),
        }
        .iter_all();

        if let Some(v) = &request.redirect_start_page {
            js.set_redirect_start_page(v)
        }
        js
    }
}

#[derive(Clone)]
pub struct SilentRequest<'a> {
    base_request: &'a BaseAuthRequest<'a>,
    account: &'a AccountInfo,
    force_refresh: Option<bool>,
    redirect_uri: Option<Cow<'a, str>>,
}

impl<'a> SilentRequest<'a> {
    pub fn from_account_info(
        base_request: &'a BaseAuthRequest<'a>,
        account_info: &'a AccountInfo,
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
        let js = msal::SilentRequest::new(
            &JsArrayString::from(&request.base_request.scopes).into(),
            request.account.into(),
        );

        IterBaseAuthRequest {
            base_auth_request: request.base_request,
            destination: &js,
            authority: &|js, v| js.set_authority(v),
            correlation_id: &|js, v| js.set_correlation_id(v),
        }
        .iter_all();

        request
            .force_refresh
            .into_iter()
            .for_each(|v| js.set_force_refresh(v));

        request
            .redirect_uri
            .iter()
            .for_each(|v| js.set_redirect_uri(v));
        js
    }
}

#[derive(Default, Clone)]
pub struct EndSessionRequest<'a> {
    account: Option<&'a AccountInfo>,
    post_logout_redirect_uri: Option<Cow<'a, str>>,
    authority: Option<Cow<'a, str>>,
    correlation_id: Option<Cow<'a, str>>,
}

impl<'a> EndSessionRequest<'a> {
    pub fn set_account(mut self, account: &'a AccountInfo) -> Self {
        self.account = Some(account);
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
        request.account.iter().for_each(|&v| {
            r.set_account(v.into());
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
        BaseAuthRequest::from(&[SCOPE][..])
            .set_authority(AUTHORITY)
            .set_correlation_id(CORRELATION_ID)
    }

    #[wasm_bindgen_test]
    fn mirror_silent_request() {
        let base_req = base_req();
        let account = account();
        let req = SilentRequest::from_account_info(&base_req, &account)
            .set_force_refresh(FORCE_REFRESH)
            .set_redirect_uri(REDIRECT_URI);

        let js_req: msal::SilentRequest = (&req).into();

        assert_eq!(
            req.base_request.scopes,
            JsArrayString::from(js_req.scopes()).0
        );
        assert_eq!(
            req.base_request.correlation_id.as_deref().map(String::from),
            js_req.correlation_id()
        );
        assert_eq!(
            req.base_request.authority.as_deref().map(String::from),
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
        let account = account();
        let req = EndSessionRequest::default()
            .set_account(&account)
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
