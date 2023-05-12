import { Component } from "react";
import { invoke, event } from "@tauri-apps/api";
import { CustomProvider, Table, Button, Divider } from "rsuite";
import NetworkStream from "./classes/stream";

import "./HttpSpy.css";
import "rsuite/styles/index.less";

import restoreIcon from './assets/restore.png';
import closeIcon from './assets/close.png';
import maxIcon from './assets/max.png';
import minIcon from './assets/min.png';
import icon from "./assets/icon.png";

let started = false;

class HttpSpy extends Component {

    constructor(props) {
        super(props);

        this.state = {
            data: [],
            current: null,
        }

        event.listen("http-data", (req) => {
            let data = req.payload;
            let stream = new NetworkStream( data );

            let packet = {
                url: stream.readString16(),
                host: stream.readString16(),
                method: stream.readString8(),
                protocol: stream.readString8()
            };

            let body_len = stream.readInt64LE();
            let buffer = [];
            for (let byte = 0; byte < body_len; byte++) {
                buffer.push( stream.readByte() );
            }
            packet.body = buffer;

            let num_headers = stream.readInt64LE();
            let headers = {};
            for (let header = 0; header < num_headers; num_headers++) {
                let key = stream.readString64();
                let value = stream.readString64();

                headers[ key ] = value;
            }
            packet.headers = headers;

            let req_data = self.state.data;
            req_data.push( packet );
            console.log(packet);
            this.setState({
                data: req_data
            })
        });

        this.onRowClick = this.onRowClick.bind(this);
    }

    onRowClick(row) {
        this.setState({
            current: row
        })
    }

    componentDidMount() {
        {
            if (!started) {
                started = true;
                console.log("started");
                (async () => {
                    await invoke(
                        "register_server",
                        {
                            serverInfo: {
                                server_type: "http",
                                server_port: 0
                            }
                        }
                    )
                })();
            }
        }
    }

    render() {
        return <CustomProvider id="wrapper" theme="dark">
            <div id='title'>
                <p id='title-text'>HTTP Spy</p>
                <div id='title-left'>
                    <div id='icon-wrapper'>
                        <img id='icon' src={icon} />
                    </div>
                </div>
                <div id='title-right'>
                    <div id='button-wrapper'>
                        <button id='min'><img id ='min-png' src={minIcon} onClick={this.minimize}/></button>
                        <button id='max'><img id='max-png' src={maxIcon} onClick={this.maximize}/></button>
                        <button id='close'><img id='close-png' src={closeIcon} onClick={this.close}/></button>
                    </div>
                </div>
            </div>
            <div id='drag' data-tauri-drag-region></div>
            <div id="main">
                <div id="main-content">
                    <div id="requests-wrapper">
                        <Table shouldUpdateScroll={() => true} data={this.state.data} onRowClick={this.onRowClick} fillHeight virtualized>
                            <Table.Column width={65} resizable>
                                <Table.HeaderCell>Method</Table.HeaderCell>
                                <Table.Cell dataKey="method" />
                            </Table.Column>
                            <Table.Column flexGrow={2} fullText resizable>
                                <Table.HeaderCell>Host</Table.HeaderCell>
                                <Table.Cell dataKey="host" />
                            </Table.Column>
                            <Table.Column  flexGrow={2} fullText resizable>
                                <Table.HeaderCell>Url</Table.HeaderCell>
                                <Table.Cell dataKey="url" />
                            </Table.Column>
                        </Table>    
                    </div>
                    <div id="inspect-wrapper">
                        <div id="inspect-type">
                            <Button class="inspect-button">Headers</Button>
                            <Button class="inspect-button">Body</Button>
                        </div>
                        <div id="inspect-headers">
                            <p class="inspect-header-title">General</p>
                            <p class="inspect-header">
                                <label>Request URL: </label>{
                                        this.state.current ?
                                        this.state.current.protocol + "://" +
                                        this.state.current.host +
                                        this.state.current.url : ""
                                    }
                            </p>
                            <p class="inspect-header">
                                <label>Request Method: </label>{ this.state.current ? this.state.current.method : "" }
                            </p>
                            <Divider />
                            <p class="inspect-header-title">Request Headers</p>
                            {
                                this.state.current ?
                                Object.keys( this.state.current.headers ).map(key => {
                                    return <p class="inspect-header">
                                        <label>{key}: </label>{this.state.current.headers[key]}
                                    </p>
                                }) : ""
                            }
                        </div>
                    </div>
                </div>
            </div>
        </CustomProvider>
    }
}

export default HttpSpy;