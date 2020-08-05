use crate::{
    acquire_token_silent, msal,
    requests::{AuthorizationUrlRequest, SilentRequest},
    sso_silent, AuthenticationResult, BrowserAuthOptions, Configuration, PublicClientApplication,
};
use wasm_bindgen::JsValue;

pub struct PopupApp {
    auth: msal::PublicClientApplication,
}

impl PublicClientApplication for PopupApp {
    fn auth(&self) -> &msal::PublicClientApplication {
        &self.auth
    }
}

impl PopupApp {
    pub fn new(configuration: Configuration) -> Self {
        Self {
            auth: msal::PublicClientApplication::new(configuration.into()),
        }
    }

    pub async fn login_popup(&self) -> Result<AuthenticationResult, JsValue> {
        self.auth
            .login_popup(Self::empty_request())
            .await
            .map(Into::into)
    }

    pub async fn login_popup_with_scopes(
        &self,
        scopes: Vec<String>,
    ) -> Result<AuthenticationResult, JsValue> {
        self.auth.login_popup(scopes.into()).await.map(Into::into)
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

    pub async fn acquire_token_popup(
        &self,
        request: AuthorizationUrlRequest,
    ) -> Result<AuthenticationResult, JsValue> {
        self.auth
            .acquire_token_popup(request.into())
            .await
            .map(Into::into)
    }
}

impl<'a> From<&'a str> for PopupApp {
    fn from(client_id: &'a str) -> Self {
        Self::new(client_id.into())
    }
}

impl<'a> From<Configuration> for PopupApp {
    fn from(configuration: Configuration) -> Self {
        PopupApp::new(configuration)
    }
}

impl<'a> From<BrowserAuthOptions> for PopupApp {
    fn from(browser_auth_options: BrowserAuthOptions) -> Self {
        Configuration::new(browser_auth_options).into()
    }
}

#[cfg(test)]
mod tests_in_browser {
    wasm_bindgen_test_configure!(run_in_browser);

    use super::*;
    use wasm_bindgen_test::*;

    const CLIENT_ID: &str = "MY_CLIENT_ID";
    const AUTHORITY: &str = "MY_AUTHORITY";
    const REDIRECT_URI: &str = "MY_REDIRECT_URI";

    #[wasm_bindgen_test]
    fn build_pub_client_full() {
        let b = BrowserAuthOptions::new(CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let c = Configuration::new(b);
        let client_app = PopupApp::new(c);
        assert_eq!(client_app.client_id(), CLIENT_ID);
        assert_eq!(client_app.authority(), AUTHORITY);
        assert_eq!(client_app.redirect_uri(), REDIRECT_URI);
    }

    #[wasm_bindgen_test]
    fn build_pub_client_from_config() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = PopupApp::from(config);
        assert_eq!(client_app.client_id(), CLIENT_ID);
    }

    #[wasm_bindgen_test]
    fn build_pub_client_from_string() {
        let client_app = PopupApp::from(CLIENT_ID);
        assert_eq!(client_app.client_id(), CLIENT_ID);
    }

    // How to correcly test these? Since require user input...
    // supress the warning for now
    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    async fn login_popup() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = PopupApp::from(config);
        client_app.login_popup();
    }
}