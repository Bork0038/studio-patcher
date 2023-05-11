import { Component } from "react";
import { CustomProvider, Table, Button, Divider } from "rsuite";

import "./HttpSpy.css";
import "rsuite/styles/index.less";

import restoreIcon from './assets/restore.png';
import closeIcon from './assets/close.png';
import maxIcon from './assets/max.png';
import minIcon from './assets/min.png';
import icon from "./assets/icon.png";

class HttpSpy extends Component {

    constructor(props) {
        super(props);

        this.state = {
            data: [
                {
                    method: "GET",
                    host: "example.com",
                    url: "/index.html",
                    protocol: "https",
                    headers: {
                        "Origin": "https://example.com/a"
                    }
                }
            ],
            current: null,
        }

        this.onRowClick = this.onRowClick.bind(this);
    }

    onRowClick(row) {
        this.setState({
            current: row
        })
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