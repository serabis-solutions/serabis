export interface ApiOptions {
    base_url : string;
    username?: string;
    password?: string;
}

export interface ApiRequestCallback {
    error?  : (jqXHR: JQueryXHR, textStatus: string, errorThrown: string) => any;
    success : (data: any) => any;
}

export interface ApiRequestOptions extends ApiRequestCallback {
    path: string;
    data?: any;
}

export abstract class Api {
    base_url    : string;
    username    : string;
    password    : string;
    api_version : number;
    url         : string;

    constructor( options : ApiOptions ) {
        this.base_url = options.base_url;
        this.username = options.username;
        this.password = options.password;

        this.api_version = 0.01;
        this.url = `${this.base_url}/${this.api_version}`;
    }

    request( options: ApiRequestOptions ) {
        let error: any;
        if ( !options.hasOwnProperty("error") ) {
            error = ( jqXHR: JQueryXHR, textStatus: string, errorThrown: string ) => {
                console.log( textStatus );
                console.log( errorThrown );
            };
        } else {
            error = options.error;
        }
        $.ajax( {
            url     : this.url + options.path,
            error   : error,
            success : (data: any, textStatus: string, jqXHR: JQueryXHR) => {
                options.success( data );
            },
            headers: {
                Authorization: `Basic ${btoa( `${this.username}:${this.password}` )}`
            },
            data : options.data
        } );
    }
}
