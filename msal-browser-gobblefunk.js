/**
 * @param {string} clientId
 */
export class BrowserAuthOptions {
    constructor(clientId) {
        this.clientId = clientId;
    }
}

/**
 * @param {BrowserAuthOptions} browserAuthOptions
 */
export class Configuration {
    constructor(browserAuthOptions) {
        this.auth = browserAuthOptions;
    }
}

/**
 * @param {[string]} scopes
 */
export class AuthorizationUrlRequest {
    constructor(scopes) {
        this.scopes = scopes;
    }
}

/**
 * @param {[string]} scopes
 */
export class RedirectRequest {
    constructor(scopes) {
        this.scopes = scopes;
    }
}

/**
 * @param {string} homeAccountId
 * @param {string} environment
 * @param {string} tenantId
 * @param {string} username
 */
export class AccountInfo {
    constructor(homeAccountId, environment, tenantId, username) {
        this.homeAccountId = homeAccountId;
        this.environment = environment;
        this.tenantId = tenantId;
        this.username = username;
    }
}

/**
 * @param {AccountInfo} account
 * @param {string} postLogoutRedirectUri
 * @param {string} authority
 * @param {string} correlationId
 */
export class EndSessionRequest {
    account;
    postLogoutRedirectUri;
    authority;
    correlationId;
};