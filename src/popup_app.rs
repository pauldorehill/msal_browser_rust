use crate::{
    acquire_token_silent, msal,
    msal::Msal,
    requests::{AuthorizationUrlRequest, SilentRequest},
    sso_silent, AuthenticationResult, BrowserAuthOptions, Configuration, PublicClientApplication,
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

impl<'a> From<&'a str> for PopupApp {
    fn from(client_id: &'a str) -> Self {
        Self::new(client_id.into())
    }
}

impl<'a> From<Configuration<'a>> for PopupApp {
    fn from(configuration: Configuration) -> Self {
        PopupApp::new(configuration)
    }
}

impl<'a> From<BrowserAuthOptions<'a>> for PopupApp {
    fn from(browser_auth_options: BrowserAuthOptions) -> Self {
        Configuration::from(browser_auth_options).into()
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

    #[wasm_bindgen_test]
    fn build_pub_client_full() {
        let b = BrowserAuthOptions::from(CLIENT_ID)
            .set_authority(AUTHORITY)
            .set_redirect_uri(REDIRECT_URI);
        let c = Configuration::from(b);
        let client_app = PopupApp::new(c);
        assert_eq!(client_app.client_id(), CLIENT_ID);
        assert_eq!(client_app.authority().unwrap(), AUTHORITY);
        assert_eq!(client_app.redirect_uri().unwrap(), REDIRECT_URI);
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

    // How to correctly test these? Since require user input...
    // suppress the warning for now
    #[allow(unused_must_use)]
    #[wasm_bindgen_test]
    async fn login_popup() {
        let config = Configuration::from(CLIENT_ID).set_authority(AUTHORITY);
        let client_app = PopupApp::from(config);
        client_app.login_popup();
    }
}
