use crate::{msal, AccountInfo};
use msal::{JsHashMapStrStr, JsArrayString};
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

// TODO: Add clone out method for setting js object values
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

// TODO: Add clone out method for setting js object values since this is used by redirect request
pub struct AuthorizationUrlRequest<'a> {
    base_request: Cow<'a, BaseAuthRequest<'a>>, // Cow here to allow both types of From<..>
    redirect_uri: Option<Cow<'a, str>>,
    extra_scopes_to_consent: Option<Vec<Cow<'a, str>>>,
    response_mode: Option<ResponseMode>,
    code_challenge: Option<Cow<'a, str>>,
    code_challenge_method: Option<Cow<'a, str>>,
    state: Option<Cow<'a, str>>,
    prompt: Option<Cow<'a, str>>,
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

impl<'a> From<&'a AuthorizationUrlRequest<'a>> for msal::AuthorizationUrlRequest {
    fn from(request: &'a AuthorizationUrlRequest) -> Self {
        let auth_req = msal::AuthorizationUrlRequest::new(
            &JsArrayString::from(&request.base_request.scopes).into(),
        );

        request
            .base_request
            .authority
            .iter()
            .for_each(|v| auth_req.set_authority(&v));

        request
            .base_request
            .correlation_id
            .iter()
            .for_each(|v| auth_req.set_correlation_id(&v));

        request
            .redirect_uri
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_redirect_uri(v));

        request
            .extra_scopes_to_consent
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_extra_scopes_to_consent(JsArrayString::from(v).into()));

        request
            .response_mode
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_response_mode(v.as_str()));

        request
            .code_challenge
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_code_challenge(v));

        request
            .code_challenge_method
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_code_challenge_method(v));

        request
            .state
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_state(v));

        request
            .prompt
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_prompt(v));

        request
            .login_hint
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_login_hint(v));

        request
            .domain_hint
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_domain_hint(v));

        request
            .extra_query_parameters
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_extra_query_parameters(JsHashMapStrStr::from(v).into()));

        request
            .claims
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_claims(v));

        request
            .nonce
            .as_ref()
            .into_iter()
            .for_each(|v| auth_req.set_nonce(v));

        auth_req
    }
}

impl<'a, T> From<&'a [T]> for AuthorizationUrlRequest<'a>
where
    T: Into<Cow<'a, str>> + std::clone::Clone,
{
    fn from(scopes: &'a [T]) -> Self {
        Self {
            base_request: Cow::Owned(scopes.into()),
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
    // TODO: Add in all fields
    fn from(request: &'a RedirectRequest<'a>) -> Self {
        let auth_req = msal::RedirectRequest::new(
            &JsArrayString::from(&request.auth_url_req.base_request.scopes).into(),
        );
        auth_req
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
        let r = msal::SilentRequest::new(
            &JsArrayString::from(&request.base_request.scopes).into(),
            request.account.into(),
        );
        request
            .base_request
            .authority
            .iter()
            .for_each(|v| r.set_authority(v));
        request
            .base_request
            .correlation_id
            .iter()
            .for_each(|v| r.set_correlation_id(v));
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
