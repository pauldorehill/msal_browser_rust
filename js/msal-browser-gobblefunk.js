export class BrowserAuthOptions {
    constructor(clientId) {
        this.clientId = clientId;
    }
}

export class CacheOptions {}
export class LoggerOptions {}
export class BrowserSystemOptions {}

export class Configuration {
    constructor(browserAuthOptions) {
        this.auth = browserAuthOptions;
    }
}

export class AccountInfo {
    constructor(homeAccountId, environment, tenantId, username) {
        this.homeAccountId = homeAccountId;
        this.environment = environment;
        this.tenantId = tenantId;
        this.username = username;
    }
}

export class AuthorizationUrlRequest {
    constructor(scopes) {
        this.scopes = scopes;
    }
}

export class RedirectRequest {
    constructor(scopes) {
        this.scopes = scopes;
    }
}

export class EndSessionRequest {}

export class SilentRequest {
    constructor(scopes, account) {
        this.scopes = scopes;
        this.account = account;
    }
}