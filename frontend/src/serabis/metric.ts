import * as api from "./api";

export interface AgentOptions extends api.ApiOptions {
    agent_key: string;
}

export interface MetricTypeOptions extends api.ApiRequestCallback {
    type: string;
}

export interface LoadMetricOptions extends api.ApiRequestCallback, MetricTypeOptions {
    key: string|Array<string>;
    start: number;
}

export class Metric extends api.Api {
    agent_key: string;

    constructor( options: AgentOptions ) {
        super( options );
        this.agent_key = options.agent_key;
    }

    list_types( options: api.ApiRequestCallback ) {
        this.request( {
            path : `/metrics/types/${this.agent_key}`,
            error : options.error,
            success: options.success
        } )
    }

    list_subkeys( options: MetricTypeOptions ) {
        this.request( {
            path : `/metrics/sample/${this.agent_key}/${options.type}`,
            error : options.error,
            success: ( data: any ) => {
                //data is a number of samples. we want all the keys, so merge them
                let merged = {};
                $.each( data, function() {
                    $.extend( true, merged, this );
                } );

                options.success( merged );
            }
        } )
    }

    load ( options: LoadMetricOptions ) {
        this.request( {
            path    : `/metrics/load/${this.agent_key}/${options.type}`,
            error   : options.error,
            success : options.success,
            data    : {
                start   : options.start,
                key     : options.key
            }
        } );
    }
}
