use js_sys::{Array, Object};
use paste::paste;
use std::convert::{TryFrom, TryInto};
use wasm_bindgen::{JsCast, JsValue};

// https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
// https://docs.microsoft.com/en-us/azure/active-directory/develop/active-directory-optional-claims#configuring-directory-extension-optional-claims
// https://tools.ietf.org/html/rfc7519#section-4.1
// https://tools.ietf.org/html/rfc7515
// https://www.iana.org/assignments/jwt/jwt.xhtml#claims

macro_rules! generate_claims {
    ( $( ($i:ident, $t:ty) ),+ ) => {

        /// Covers all the claims as per the  IETF spec. If the claim doesn't match any of the standard ones
        /// it will return `Custom::(claim_name, claim_value)`
        /// Adds the azure specific ones too
        #[derive(Clone, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum TokenClaim {
            typ, // Always JWT
            $(
                $i ($t),
            )+
            custom(String, JsValue), // Custom to cover all else
        }

        impl TryFrom<JsValue> for TokenClaim {
            type Error = (String, JsValue);

            fn try_from(js_value: JsValue) -> Result<Self, Self::Error> {
                let kv = js_value.unchecked_into::<Array>();
                let value = kv.get(1);
                let key: String = kv.get(0).as_string().unwrap();
                let make_string = |f: &dyn Fn(String) -> Self, key, value: JsValue| match value.as_string()
                {
                    Some(value) => Ok(f(value)),
                    None => Err((key, value)),
                };
                let make_f64 = |f: &dyn Fn(f64) -> Self, key, value: JsValue| match value.as_f64() {
                    Some(value) => Ok(f(value.to_owned())),
                    None => Err((key, value)),
                };
                let make_bool = |f: &dyn Fn(bool) -> Self, key, value: JsValue| match value.as_bool() {
                    Some(value) => Ok(f(value)),
                    None => Err((key, value)),
                };
                let make_array = |f: &dyn Fn(Array) -> Self, key, value: JsValue| match value.dyn_into() {
                    Ok(value) => Ok(f(value)),
                    Err(value) => Err((key, value)),
                };
                let make_object = |f: &dyn Fn(Object) -> Self, key, value: JsValue| {
                    if value.is_object() {
                        Ok(f(value.unchecked_into()))
                    } else {
                        Err((key, value))
                    }
                };

                // Returned keys are always strings
                paste! {
                    match key.as_str() {
                        "typ" => Ok(Self::typ), // Always in JWT
                            $(
                                stringify!($i) => [< make_ $t:lower >] (&Self::$i, key, value),
                            ) +
                            // Catch anything not matched
                            _ => Ok(Self::custom(key, value)),
                            }
                }
            }
        }
    }
}

generate_claims! {
     (nonce, String),
     (alg, String),
     (kid, String),
     (x5t, String),
     (iss, String),
     (sub, String),
     (aud, String),
     (exp, f64),
     (nbf, f64),
     (iat, f64),
     (jti, String),
     (name, String),
     (given_name, String),
     (family_name, String),
     (middle_name, String),
     (nickname, String),
     (preferred_username, String),
     (profile, String),
     (picture, String),
     (website, String),
     (email, String),
     (email_verified, bool),
     (gender, String),
     (birthdate, String),
     (zoneinfo, String),
     (locale, String),
     (phone_number, String),
     (phone_number_verified, bool),
     (address, Object),
     (updated_at, f64),
     (cnf, Object),
     (sip_from_tag, String),
     (sip_date, f64),
     (sip_callid, String),
     (sip_cseq_num, String),
     (sip_via_branch, String),
     (orig, Object),
     (dest, Object),
     (mky, Object),
     (events, Object),
     (toe, f64),
     (txn, String),
     (rph, Object),
     (sid, String),
     (vot, String),
     (vtm, String),
     (attest, String),
     (origid, String),
     (act, Object),
     (scope, String),
     (client_id, String),
     (may_act, Object),
     (jcard, Object),
     (at_use_nbr, f64), // Technically u32?
     (div, Object),
     (opt, String),
     // Azure custom
     (idp, String),
     (ver, String),
     (oid, String),
     (tid, String),
     (aio, String),
     (azp, String),
     (azpacr, String),
     (rh, String),
     (scp, String),
     (uti, String),
     (appid, String),
     (roles, Array),
     (wids, Array),
     // TODO: groups:src1 // _claim_sources?
     (groups, Array),
     (hasgroups, bool)
}

#[derive(Clone)]
pub struct TokenClaims(pub Vec<TokenClaim>);

impl From<Object> for TokenClaims {
    fn from(js_obj: Object) -> Self {
        let mut claims = Vec::new();
        js_sys::Object::entries(&js_obj).for_each(&mut |v, _, _| {
            // If the expected type doesn't match do not return the claim
            if let Ok(v) = v.try_into() {
                claims.push(v)
            };
        });
        Self(claims)
    }
}

//TODO: Add an api for this
impl TokenClaims {}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen(module = "/msal-object-examples.js")]
    extern "C" {
        static accessToken: Object;
        static idToken: Object;
        static completeToken: Object;
    }

    #[wasm_bindgen_test]
    fn parse_access_token() {
        let _: TokenClaims = accessToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn parse_id_token() {
        let _: TokenClaims = idToken.clone().into();
    }

    #[wasm_bindgen_test]
    fn parse_claims() {
        let id_claims: TokenClaims = idToken.clone().into();
        let access_claims: TokenClaims = accessToken.clone().into();
        let claim = id_claims
            .0
            .iter()
            .find_map(|v| {
                if let TokenClaim::alg(c) = v {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(claim, "RS256");

        let no_custom = |claims: TokenClaims| {
            claims.0.into_iter().find_map(|v| {
                if let TokenClaim::custom(c, v) = v {
                    Some((c, v))
                } else {
                    None
                }
            })
        };

        let all: TokenClaims = completeToken.clone().into();

        // Check have found all azure claims, the source may not have them all though!
        assert!(no_custom(id_claims).is_none());
        assert!(no_custom(access_claims).is_none());
        assert!(no_custom(all).is_none());
    }
}
