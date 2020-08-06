use crate::{
    acquire_token_silent, msal,
    requests::{AuthorizationUrlRequest, RedirectRequest, SilentRequest},
    sso_silent, AuthenticationResult, Configuration, PublicClientApplication,
};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

// TODO: Should i remove the on error since is only ever msal-browser error
pub struct RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult),
    // FErr: Fn(JsValue),
{
    auth: msal::PublicClientApplication,
    on_redirect_success: FSuccess,
    // on_redirect_error: Option<FErr>,
}

impl<FSuccess> PublicClientApplication for RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult),
    // FErr: Fn(JsValue),
{
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl<FSuccess> RedirectApp<FSuccess>
where
    FSuccess: Fn(AuthenticationResult),
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
        self.login_redirect_with_scopes(vec![]).await
    }

    pub async fn login_redirect_with_scopes(&self, scopes: Vec<String>) {
        match self.auth.handle_redirect_promise().await {
            Ok(auth_result) => {
                // AuthenticationResult will be undefined / null if not a redirect
                // Can't use the 'safe' methods since the type check fails even when valid - not an object?
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

    pub async fn acquire_token_redirect(&self, request: RedirectRequest) {
        self.auth.acquire_token_redirect(request.into())
    }

    pub async fn sso_silent(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        sso_silent(&self.auth, request).await
    }

    pub async fn acquire_token_silent(
        &self,
        request: SilentRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        acquire_token_silent(&self.auth, request).await
    }
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;
    
    const CLIENT_ID: &str = "MY_CLIENT_ID";
    const AUTHORITY: &str = "MY_AUTHORITY";
    const REDIRECT_URI: &str = "MY_REDIRECT_URI";

    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    fn login_redirect() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = RedirectApp::new(config, |_| ());
        client_app.login_redirect();
    }
}
