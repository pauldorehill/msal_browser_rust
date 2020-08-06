// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md

// The configuration object has the following structure, and can be passed into the PublicClientApplication constructor. 
// The only required config parameter is the client ID of the application. 
// Everything else is optional, but may be required depending on your tenant and application model.

const LogLevel = {
    Error: undefined,
    Info: undefined,
    Verbose: undefined,
    Warning: undefined,
}

const msalConfig = {
    auth: {
        clientId: "enter_client_id_here",
        authority: "https://login.microsoftonline.com/common",
        knownAuthorities: [],
        cloudDiscoveryMetadata: "",
        redirectUri: "enter_redirect_uri_here",
        postLogoutRedirectUri: "enter_postlogout_uri_here",
        navigateToLoginRequestUrl: true
    },
    cache: {
        cacheLocation: "sessionStorage",
        storeAuthStateInCookie: false
    },
    system: {
        loggerOptions: {
            /**
         * @param {LogLevel} level
         * @param {string} message
         * @param {boolean} containsPii
         */
            loggerCallback: (level, message, containsPii) => {
                if (containsPii) {
                    return;
                }
                switch (level) {
                    case LogLevel.Error:
                        console.error(message);
                        return;
                    case LogLevel.Info:
                        console.info(message);
                        return;
                    case LogLevel.Verbose:
                        console.debug(message);
                        return;
                    case LogLevel.Warning:
                        console.warn(message);
                        return;
                }
            },
            piiLoggingEnabled: false
        },
        windowHashTimeout: 60000,
        iframeHashTimeout: 6000,
        loadFrameTimeout: 0
    },
}

// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/request-response-object.md
const authResponse = {
    uniqueId: "uniqueId",
    tenantId: "tenantId",
    scopes: ["scopes"],
    account: {
        homeAccountId: "homeAccountId",
        environment: "environment",
        tenantId: "tenantId",
        username: "username",
    },
    idToken: "idToken",
    //file://./../node_modules/@azure/msal-common/dist/src/utils/MsalTypes.d.ts
    idTokenClaims: { "typ": "JWT", },
    accessToken: "accessToken",
    fromCache: true,
    expiresOn: "Thu Aug 06 2020 10:35:12 GMT+1000 (Australian Eastern Standard Time)",
    extExpiresOn: "Thu Aug 06 2020 10:35:12 GMT+1000 (Australian Eastern Standard Time)",
    state: "state",
    familyId: "familyId",
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens
const accessToken = {
    "typ": "JWT",
    "alg": "RS256",
    "kid": "i6lGk3FZzxRcUb2C3nEQ7syHJlY",
    "aud": "6e74172b-be56-4843-9ff4-e66a39bb12e3",
    "iss": "https://login.microsoftonline.com/72f988bf-86f1-41af-91ab-2d7cd011db47/v2.0",
    "iat": 1537231048,
    "nbf": 1537231048,
    "exp": 1537234948,
    "aio": "AXQAi/8IAAAAtAaZLo3ChMif6KOnttRB7eBq4/DccQzjcJGxPYy/C3jDaNGxXd6wNIIVGRghNRnwJ1lOcAnNZcjvkoyrFxCttv33140RioOFJ4bCCGVuoCag1uOTT22222gHwLPYQ/uf79QX+0KIijdrmp69RctzmQ==",
    "azp": "6e74172b-be56-4843-9ff4-e66a39bb12e3",
    "azpacr": "0",
    "name": "Abe Lincoln",
    "oid": "690222be-ff1a-4d56-abd1-7e4f7d38e474",
    "preferred_username": "abeli@microsoft.com",
    "rh": "I",
    "scp": "access_as_user",
    "sub": "HKZpfaHyWadeOouYlitjrI-KffTm222X5rrV3xDqfKQ",
    "tid": "72f988bf-86f1-41af-91ab-2d7cd011db47",
    "uti": "fqiBqXLPj0eQa82S-IYFAA",
    "ver": "2.0"
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/id-tokens
const idToken = {
    "typ": "JWT",
    "alg": "RS256",
    "kid": "1LTMzakihiRla_8z2BEJVXeWMqo",
    "ver": "2.0",
    "iss": "https://login.microsoftonline.com/9122040d-6c67-4c5b-b112-36a304b66dad/v2.0",
    "sub": "AAAAAAAAAAAAAAAAAAAAAIkzqFVrSaSaFHy782bbtaQ",
    "aud": "6cb04018-a3f5-46a7-b995-940c78f5aef3",
    "exp": 1536361411,
    "iat": 1536274711,
    "nbf": 1536274711,
    "name": "Abe Lincoln",
    "preferred_username": "AbeLi@microsoft.com",
    "oid": "00000000-0000-0000-66f3-3332eca7ea81",
    "tid": "9122040d-6c67-4c5b-b112-36a304b66dad",
    "nonce": "123523",
    "aio": "Df2UVXL1ix!lMCWMSOJBcFatzcGfvFGhjKv8q5g0x732dR5MB5BisvGQO7YWByjd8iQDLq!eGbIDakyp5mnOrcdqHeYSnltepQmRp6AIZ8jY"
}

export { msalConfig, authResponse, accessToken, idToken }