import * as api from "./api";

export interface AccountOptions extends api.ApiOptions {
    account_key: string;
}

export class Account extends api.Api {
    account_key: string;

    constructor( options: AccountOptions ) {
        super( options );
        this.account_key = options.account_key;
    }

    list_agents( options: api.ApiRequestCallback ) {
        this.request( {
            path : `/agents/${this.account_key}`,
            error : options.error,
            success: options.success
        } )
    }
}
