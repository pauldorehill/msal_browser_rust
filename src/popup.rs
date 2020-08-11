use crate::{
    acquire_token_silent, msal,
    msal::Msal,
    requests::{AuthorizationUrlRequest, SilentRequest},
    sso_silent, AuthenticationResult, Configuration, PublicClientApplication,
};
use wasm_bindgen::JsValue;

pub struct PopupApp {
    auth: msal::PublicClientApplication,
}

impl Msal for PopupApp {
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl PublicClientApplication for PopupApp {}

impl PopupApp {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            auth: msal::PublicClientApplication::new(configuration.into()),
        }
    }

    pub async fn login_popup(&self) -> Result<AuthenticationResult, JsValue> {
        let scopes: [&str; 0] = [];
        self.login_popup_with_scopes(&scopes).await
    }

    pub async fn login_popup_with_scopes<'a, T>(
        &self,
        scopes: &'a [T],
    ) -> Result<AuthenticationResult, JsValue>
    where
        T: Into<String> + Clone,
    {
        self.auth.login_popup(scopes.into()).await.map(Into::into)
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

    pub async fn acquire_token_popup<'a>(
        &self,
        request: &'a AuthorizationUrlRequest<'a>,
    ) -> Result<AuthenticationResult, JsValue> {
        self.auth
            .acquire_token_popup(request.into())
            .await
            .map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use crate::{tests::*, BrowserAuthOptions};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn build_pub_client_full() {
        let b = BrowserAuthOptions::new(tests::CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let c = Configuration::new(b, None, None);
        let client_app = PopupApp::new(c);
        assert_eq!(client_app.client_id(), CLIENT_ID);
        assert_eq!(client_app.authority().unwrap(), AUTHORITY);
        assert_eq!(client_app.redirect_uri().unwrap(), REDIRECT_URI);
    }

    // How to correctly test these? Since require user input...
    // suppress the warning for now
    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    async fn login_popup() {
        let b = BrowserAuthOptions::new(tests::CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let c = Configuration::new(b, None, None);
        let client_app = PopupApp::new(c);
        client_app.login_popup();
    }
}
