const LogLevel = {
    Error: undefined,
    Info: undefined,
    Verbose: undefined,
    Warning: undefined,
}

// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/configuration.md
// The configuration object has the following structure, and can be passed into the PublicClientApplication constructor. 
// The only required config parameter is the client ID of the application. 
// Everything else is optional, but may be required depending on your tenant and application model.

const auth = {
    clientId: "enter_client_id_here",
    authority: "https://login.microsoftonline.com/common",
    knownAuthorities: ["a", "b"],
    cloudDiscoveryMetadata: "cloudDiscoveryMetadata",
    redirectUri: "enter_redirect_uri_here",
    postLogoutRedirectUri: "enter_postlogout_uri_here",
    navigateToLoginRequestUrl: true
}

const cache = {
    cacheLocation: "sessionStorage",
    storeAuthStateInCookie: false
}

const system = {
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
}

const config = {
    auth: auth,
    cache: cache,
    system: system,
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
    "ver": "2.0",
    // Extras added
    "idp": "idp",
    "appid": "app_id",
    "roles": ["roles"],
    "wids": ["wids"],
    "groups": ["groups"],
    "hasgroups": true,
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

// https://github.com/AzureAD/microsoft-authentication-library-for-js/blob/dev/lib/msal-browser/docs/request-response-object.md
const authResponse = {
    uniqueId: "uniqueId",
    tenantId: "tenantId",
    scopes: [
        "openid",
        "profile",
        "email"
    ],
    account: {
        homeAccountId: "homeAccountId",
        environment: "environment",
        tenantId: "tenantId",
        username: "username",
    },
    idToken: "idToken",
    idTokenClaims: idToken,
    accessToken: "accessToken",
    fromCache: true,
    expiresOn: "Thu Aug 06 2020 10:35:12 GMT+1000 (Australian Eastern Standard Time)",
    extExpiresOn: "Thu Aug 06 2020 10:35:12 GMT+1000 (Australian Eastern Standard Time)",
    state: "state",
    familyId: "familyId",
}

const completeToken = {
    typ: "typ",
    nonce: "nonce",
    alg: "alg",
    kid: "kid",
    x5t: "x5t",
    iss: "iss",
    sub: "sub",
    aud: "aud",
    exp: 13,
    nbf: 13,
    iat: 13,
    jti: "jti",
    name: "name",
    given_name: "given_name",
    family_name: "family_name",
    middle_name: "middle_name",
    nickname: "nickname",
    preferred_username: "preferred_username",
    profile: "profile",
    picture: "picture",
    website: "website",
    email: "email",
    email_verified: true,
    gender: "gender",
    birthdate: "birthdate",
    zoneinfo: "zoneinfo",
    locale: "locale",
    phone_number: "phone_number",
    phone_number_verified: true,
    address: {},
    updated_at: 13,
    cnf: {},
    sip_from_tag: "sip_from_tag",
    sip_date: 13,
    sip_callid: "sip_callid",
    sip_cseq_num: "sip_cseq_num",
    sip_via_branch: "sip_via_branch",
    orig: {},
    dest: {},
    mky: {},
    events: {},
    toe: 13,
    txn: "txn",
    rph: {},
    sid: "sid",
    vot: "vot",
    vtm: "vtm",
    attest: "attest",
    origid: "origid",
    act: {},
    scope: "scope",
    client_id: "client_id",
    may_act: {},
    jcard: {},
    at_use_nbr: 13,
    div: {},
    opt: "opt",
    idp: "idp",
    ver: "ver",
    oid: "oid",
    tid: "tid",
    aio: "aio",
    azp: "azp",
    azpacr: "azpacr",
    rh: "rh",
    scp: "scp",
    uti: "uti",
    appid: "appid",
    roles: [],
    wids: [],
    groups: [],
    hasgroups: true,
}

export { config, authResponse, accessToken, idToken, completeToken, auth, cache, system }