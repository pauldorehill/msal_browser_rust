use crate::{
    acquire_token_silent, msal,
    msal::Msal,
    requests::{AuthorizationUrlRequest, RedirectRequest, SilentRequest},
    sso_silent, AuthenticationResult, Configuration, PublicClientApplication,
};
use wasm_bindgen::{JsCast, JsValue};

// TODO: Should i remove the on error since is only ever msal-browser error
pub struct RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult) + Clone,
    // FErr: Fn(JsValue),
{
    auth: msal::PublicClientApplication,
    on_redirect_success: FSuccess,
    // on_redirect_error: Option<FErr>,
}

impl<FSuccess> Clone for RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult) + Clone,
{
    fn clone(&self) -> Self {
        Self {
            auth: self.auth.clone().into(),
            on_redirect_success: self.on_redirect_success.clone(),
        }
    }
}

impl<FSuccess> Msal for RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult) + Clone,
{
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl<FSuccess> PublicClientApplication for RedirectApp<FSuccess> where
    FSuccess: Fn(AuthenticationResult) + Clone // FErr: Fn(JsValue),
{
}

impl<FSuccess> RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult) + Clone,
    // FErr: Fn(JsValue),
{
    pub fn new(
        configuration: Configuration,
        on_redirect_success: FSuccess,
        // on_redirect_error: Option<FErr>,
    ) -> Self {
        let auth = msal::PublicClientApplication::new(configuration.into());
        Self {
            auth,
            on_redirect_success,
            // on_redirect_error,
        }
    }

    pub async fn login_redirect(&self) {
        let empty: [&str; 0] = [];
        self.login_redirect_with_scopes(&empty).await
    }

    pub async fn login_redirect_with_scopes<'a, T>(&self, scopes: &'a [T])
    where
        T: Into<String> + Clone,
    {
        match self.auth.handle_redirect_promise().await {
            Ok(auth_result) => {
                // AuthenticationResult will be undefined / null if not a redirect
                // Can't use the 'safe' methods since the type check fails even when valid as is an Object.
                let auth_res = auth_result.unchecked_into::<msal::AuthenticationResult>();
                if auth_res.is_undefined() || auth_res.is_null() {
                    self.auth.login_redirect(scopes.into())
                } else {
                    (self.on_redirect_success)(auth_res.into())
                }
            }
            // Will always be ok unless the msal library errors?
            Err(_) => {
                // if let Some(f) = &self.on_redirect_error {
                //     f(e)
                // }
            }
        }
    }

    pub async fn acquire_token_redirect<'a>(&self, request: &'a RedirectRequest<'a>) {
        self.auth.acquire_token_redirect(request.into())
    }

    pub async fn sso_silent<'a>(
        &self,
        request: &'a AuthorizationUrlRequest<'a>,
    ) -> Result<AuthenticationResult, JsValue> {
        sso_silent(&self.auth, request).await
    }

    pub async fn acquire_token_silent<'a>(
        &self,
        request: &'a SilentRequest<'a>,
    ) -> Result<AuthenticationResult, JsValue> {
        acquire_token_silent(&self.auth, request).await
    }
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use crate::{tests::*, BrowserAuthOptions};
    use wasm_bindgen_test::*;

    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    fn login_redirect() {
        let b = BrowserAuthOptions::new(tests::CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let config = Configuration::new(b);
        let client_app = RedirectApp::new(config, |_| ());
        client_app.login_redirect();
    }
}
