use crate::{msal, AccountInfo};
use msal::{JsArrayString, JsHashMapStrStr};
use std::{borrow::Cow, collections::HashMap};

#[derive(Clone)]
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

#[derive(Clone)]
pub struct BaseAuthRequest<'a> {
    scopes: Vec<Cow<'a, str>>, // TODO: Can this be a slice?
    authority: Option<Cow<'a, str>>,
    correlation_id: Option<Cow<'a, str>>,
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

pub trait SetBaseAuthrequest<'a> {
    fn base_request(&mut self) -> &mut BaseAuthRequest<'a>;

    fn set_authority<T>(mut self, authority: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.base_request().authority = Some(authority.into());
        self
    }

    fn set_correlation_id<T>(mut self, correlation_id: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.base_request().correlation_id = Some(correlation_id.into());
        self
    }
}

impl<'a> BaseAuthRequest<'a> {
    fn new<T>(scopes: &'a [T]) -> Self
    where
        T: Clone + Into<Cow<'a, str>>,
    {
        Self {
            scopes: scopes.into_iter().cloned().map(Into::into).collect(),
            authority: None,
            correlation_id: None,
        }
    }
}

#[derive(Clone)]
pub enum Prompt {
    Login,
    None,
    Consent,
    SelectAccount,
}

impl Prompt {
    fn as_str(&self) -> &str {
        match &self {
            Prompt::Login => "login",
            Prompt::None => "none",
            Prompt::Consent => "consent",
            Prompt::SelectAccount => "select_account",
        }
    }
}
#[derive(Clone)]
pub struct AuthorizationUrlRequest<'a> {
    base_request: BaseAuthRequest<'a>,
    redirect_uri: Option<Cow<'a, str>>,
    extra_scopes_to_consent: Option<Vec<Cow<'a, str>>>,
    response_mode: Option<ResponseMode>,
    code_challenge: Option<Cow<'a, str>>,
    code_challenge_method: Option<Cow<'a, str>>,
    state: Option<Cow<'a, str>>,
    prompt: Option<Prompt>,
    login_hint: Option<Cow<'a, str>>,
    domain_hint: Option<Cow<'a, str>>,
    extra_query_parameters: Option<HashMap<Cow<'a, str>, Cow<'a, str>>>,
    claims: Option<Cow<'a, str>>,
    nonce: Option<Cow<'a, str>>,
}

impl<'a> AuthorizationUrlRequest<'a> {
    pub fn new<T>(scopes: &'a [T]) -> Self
    where
        T: Clone + Into<Cow<'a, str>>,
    {
        Self {
            base_request: BaseAuthRequest::new(scopes),
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
    prompt: &'a dyn Fn(&T, &'a str),
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
            (self.prompt)(self.destination, v.as_str())
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

pub trait SetAuthorizationUrlRequest<'a> {
    fn auth_request(&mut self) -> &mut AuthorizationUrlRequest<'a>;

    fn set_redirect_uri<T>(mut self, redirect_uri: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().redirect_uri = Some(redirect_uri.into());
        self
    }

    fn set_extra_scopes_to_consent<T>(mut self, extra_scopes_to_consent: &'a [T]) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        Self: std::marker::Sized,
    {
        self.auth_request().extra_scopes_to_consent = Some(
            extra_scopes_to_consent
                .iter()
                .cloned()
                .map(|v| v.into())
                .collect(),
        );
        self
    }

    fn set_response_mode(mut self, response_mode: ResponseMode) -> Self
    where
        Self: std::marker::Sized,
    {
        self.auth_request().response_mode = Some(response_mode);
        self
    }

    fn set_code_challenge<T>(mut self, code_challenge: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().code_challenge = Some(code_challenge.into());
        self
    }

    fn set_code_challenge_method<T>(mut self, code_challenge_method: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().code_challenge_method = Some(code_challenge_method.into());
        self
    }

    fn set_state<T>(mut self, state: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().state = Some(state.into());
        self
    }

    fn set_prompt<T>(mut self, prompt: Prompt) -> Self
    where
        Self: std::marker::Sized,
    {
        self.auth_request().prompt = Some(prompt);
        self
    }

    fn set_login_hint<T>(mut self, login_hint: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().login_hint = Some(login_hint.into());
        self
    }

    fn set_domain_hint<T>(mut self, domain_hint: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().domain_hint = Some(domain_hint.into());
        self
    }

    fn set_extra_query_parameters<T>(mut self, extra_query_parameters: HashMap<T, T>) -> Self
    where
        T: Into<Cow<'a, str>> + Clone,
        Self: std::marker::Sized,
    {
        let mut hm: HashMap<Cow<'a, str>, Cow<'a, str>> = HashMap::new();
        for (k, v) in extra_query_parameters {
            hm.insert(k.into(), v.into());
        }
        self.auth_request().extra_query_parameters = Some(hm);
        self
    }

    fn set_claims<T>(mut self, claims: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().claims = Some(claims.into());
        self
    }

    fn set_nonce<T>(mut self, nonce: T) -> Self
    where
        T: Into<Cow<'a, str>>,
        Self: std::marker::Sized,
    {
        self.auth_request().nonce = Some(nonce.into());
        self
    }
}

impl<'a> SetAuthorizationUrlRequest<'a> for AuthorizationUrlRequest<'a> {
    fn auth_request(&mut self) -> &mut AuthorizationUrlRequest<'a> {
        self
    }
}

impl<'a> SetBaseAuthrequest<'a> for AuthorizationUrlRequest<'a> {
    fn base_request(&mut self) -> &mut BaseAuthRequest<'a> {
        &mut self.base_request
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

#[cfg(feature = "redirect")]
#[derive(Clone)]
pub struct RedirectRequest<'a> {
    auth_url_req: AuthorizationUrlRequest<'a>,
    redirect_start_page: Option<Cow<'a, str>>,
}

#[cfg(feature = "redirect")]
impl<'a> RedirectRequest<'a> {
    pub fn new<T>(scopes: &'a [T]) -> Self
    where
        T: Clone + Into<Cow<'a, str>>,
    {
        Self {
            auth_url_req: AuthorizationUrlRequest::new(scopes),
            redirect_start_page: None,
        }
    }
}

#[cfg(feature = "redirect")]
impl<'a> SetBaseAuthrequest<'a> for RedirectRequest<'a> {
    fn base_request(&mut self) -> &mut BaseAuthRequest<'a> {
        &mut self.auth_url_req.base_request
    }
}

#[cfg(feature = "redirect")]
impl<'a> SetAuthorizationUrlRequest<'a> for RedirectRequest<'a> {
    fn auth_request(&mut self) -> &mut AuthorizationUrlRequest<'a> {
        &mut self.auth_url_req
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
    base_request: BaseAuthRequest<'a>,
    account: &'a AccountInfo,
    force_refresh: Option<bool>,
    redirect_uri: Option<Cow<'a, str>>,
}

impl<'a> SetBaseAuthrequest<'a> for SilentRequest<'a> {
    fn base_request(&mut self) -> &mut BaseAuthRequest<'a> {
        &mut self.base_request
    }
}

impl<'a> SilentRequest<'a> {
    pub fn new<T>(scopes: &'a [T], account_info: &'a AccountInfo) -> Self
    where
        T: Clone + Into<Cow<'a, str>>,
    {
        Self {
            base_request: BaseAuthRequest::new(scopes),
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
            base_auth_request: &request.base_request,
            destination: &js,
            authority: &|js, v| js.set_authority(v),
            correlation_id: &|js, v| js.set_correlation_id(v),
        }
        .iter_all();

        if let Some(v) = request.force_refresh {
            js.set_force_refresh(v)
        }
        if let Some(v) = &request.redirect_uri {
            js.set_redirect_uri(v)
        }
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
    pub fn new() -> Self {
        Self::default()
    }

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
        let js = msal::EndSessionRequest::new();
        if let Some(v) = request.account {
            js.set_account(v.into())
        }
        if let Some(v) = &request.post_logout_redirect_uri {
            js.set_post_logout_redirect_uri(&v)
        }
        if let Some(v) = &request.authority {
            js.set_authority(&v)
        }
        if let Some(v) = &request.correlation_id {
            js.set_correlation_id(&v)
        }
        js
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
        let _req = AuthorizationUrlRequest::new(&[SCOPE][..]);
        // TODO: Write tests
    }

    #[wasm_bindgen_test]
    fn mirror_redirect_request() {
        // TODO: Write tests
    }

    #[wasm_bindgen_test]
    fn mirror_silent_request() {
        let account = account();
        let req = SilentRequest::new(&[SCOPE][..], &account)
            .set_force_refresh(FORCE_REFRESH)
            .set_redirect_uri(REDIRECT_URI)
            .set_correlation_id(CORRELATION_ID)
            .set_authority(AUTHORITY);

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
