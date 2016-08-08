import * as serabis from "./serabis";

$( document ).ready(function() {
    $("#load_account").click( function( e ) {
        e.preventDefault();

        let account = new serabis.Account( {
            base_url    : $('#base_url').val(),
            username    : $('#username').val(),
            password    : $('#password').val(),
            account_key : $('#account_key').val()
        } );

        account.list_agents( {
            success : ( data: any ) => {
                let $device = $("#device");
                $device.empty();
                $device.append( $('<option/>', {value:"",text:"Please select a device"}) );

                $.each( data, function() {
                    let $option = $( '<option />', {
                        value : this.key,
                        text  : this.hostname
                    } );

                    $device.append( $option );
                } );

                $("#agent_form>fieldset").prop( 'disabled', false );
            }
        } )
    } );

    $("#device").change( function() {
        let value: string = $(this).val();

        if ( value ) {
            let metric = new serabis.Metric( {
                base_url    : $('#base_url').val(),
                username    : $('#username').val(),
                password    : $('#password').val(),
                agent_key   : $('#device').val()
            } );
            $("#device").data( "metric", metric );

            let $metric_type = $("#metric_type");
            metric.list_types( {
                success : ( data: any ) => {
                    $metric_type.empty();
                    $metric_type.append( $('<option/>', {value:"",text:"Please select a metric type"}) );

                    $.each( data, function() {
                        let $option = $( '<option />', {
                            value : this,
                            text  : this
                        } );

                        $metric_type.append( $option );
                    } );
                }
            } );
        }
    } );

const RECURSE_KEYS_MAX_DEPTH = 10;
function recurse_keys( value: any, depth?: number, parent_key?: string ): Array<Object>|void {
    if ( depth === undefined ) {
        depth = 0;
    }
    if ( depth > RECURSE_KEYS_MAX_DEPTH ) {
        throw new Error("recursed too deep");
    }

    if ( typeof value === 'object' ) {
        let stack :Array<Object> = [];

        for ( let key in value ) {
            if ( parent_key !== undefined ) {
                var id = `${parent_key}:${key}`
            }
            else {
                var id = key;
            }

            let children = recurse_keys( value[key], depth + 1, id );
            let node = {
                id       : id,
                text     : key,
                children : children
            }

            stack.push( node );
        }

        return stack;
    }

    return;
}

    $("#metric_type").change( function() {
        let $metric_type = $(this);
        let metric = $("#device").data("metric");

        metric.list_subkeys( {
            type    : $metric_type.val(),
            success : (data: any) => {
                let tree_data = recurse_keys( data );
                let $selector = $("#selector");

                if ( !$selector.jstree( true ) ) {
                    $selector.jstree( {
                        core: {
                            data: tree_data,
                            themes: {
                                icons: false
                            }
                        },
                        plugins : [ "wholerow", "checkbox", "sort" ]
                    } );
                }
                else {
                    $selector.jstree(true).settings.core.data = tree_data;
                    $selector.jstree(true).refresh( false, true );

                }
            }
        } );
    } );

    $("#go").click(function(e){
        e.preventDefault();
        let metric = $("#device").data("metric");

        let container = $( '<div></div>', {class:"graph_container"} );
        let container_closer = $( '<a />', {
            text  : "x",
            class : "graph_close",
            click : function() {
                $(this).parent().remove();
            }
        } );
        container.append( container_closer );

        let graph_canvas = $( '<canvas />' );
        container.append( graph_canvas );
        $("#playarea").prepend( container );

        let is_delta = $("#delta").is(':checked');
        let divider = $("#divider").val();

        metric.load( {
            type  : $('#metric_type').val(),
            start : ( parseInt( String( (new Date).getTime() / 1000 ) ) - ( $("#age").val() ) ),
            key   : $('#selector').jstree('get_selected'),
            success : (data: Array<any>) => {
                let datasets        : any           = {};
                let labels          : Array<Object> = [];
                let previous_values : any           = {};

                for ( let i=0; i < data.length; i++ ) {
                    let row = data[i];
                    let ts = row.ts;
                    if ( ! is_delta ) {
                        labels.push( ts );
                    }
                    else if ( is_delta && ( i !== 0 ) ) {
                        labels.push( ts );
                    }

                    for ( let key in row.metrics ) {
                        if ( datasets[key] === undefined ) {
                            datasets[key] = [];
                        }

                        let value = row.metrics[key] / divider;

                        if ( is_delta && ( i === 0 ) ) {
                            previous_values[key] = value;
                            continue;
                        }

                        if ( is_delta ) {
                            var display_value = ( value - previous_values[key] );
                        }
                        else {
                            var display_value = value;
                        }

                        let point = {
                            x: ts,
                            y: display_value.toFixed(2)
                        };

                        datasets[key].push( point );

                        if ( is_delta ) {
                            previous_values[key] = value;
                        }
                    }
                }

                let ctx = <HTMLCanvasElement>graph_canvas[0];
                let myLineChart = new Chart( ctx.getContext('2d'), {
                    type: 'line',
                    data: {
                        labels : labels,
                        datasets: $.map( datasets, (value, key) => {
                            return {
                                label   : key,
                                data    : value
                            }

                        } )
                    }
                });
            }
            
        } );
    } );
});
