import { useEffect, useState } from "react";
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
function HttpSpy(props) {
    const [data, setData] = useState([]);
    const [current, setCurrent] = useState(null);

    const onRowClick = (row) => {
        setCurrent(row);
    };

    const getData = () => {
        return data;
    }

    const close = async () => {
		await process.exit();
	}

	const minimize = async () => {
		await window
			.getCurrent()
			.minimize();
	}

	const maximize = async () => {
		const currentWindow = window.getCurrent();
		const isMaximized   = await currentWindow.isMaximized();

		document.getElementById('max-png').src = isMaximized ? maxIcon : restoreIcon;
		isMaximized ? currentWindow.unmaximize() : currentWindow.maximize();

		this.setState();
	}

    useEffect(() => {
        if (!started) {
            started = true;

            (async () => {
                await invoke("register_server", {
                    serverInfo: {
                        server_type: "http",
                        server_port: 0,
                    },
                });
            })();
        }

        const listen = event.listen("http-data", (req) => {
            let payload = req.payload;
            let stream = new NetworkStream( payload );

            let packet = {
                url: stream.readString16(),
                host: stream.readString16(),
                method: stream.readString8(),
                protocol: stream.readString8(),
            };

            let body_len = stream.readInt32LE();
            let buffer = [];
            for (let byte = 0; byte < body_len; byte++) {
                buffer.push(stream.readByte());
            }
            packet.body = buffer;

            let num_headers = stream.readInt32LE();
            let headers = {};
            for (let header = 0; header < num_headers; header++) {
                let key = stream.readString32();
                let value = stream.readString32();

                headers[key] = value;
            }
            packet.headers = headers;
            packet.key = data.length;

            let req_data = [...data];
            req_data.push(packet);

            setData(req_data);
        });

        return () => {
            listen.then((f) => f());
        };
    });

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
                    <button id='min'><img id='min-png' src={minIcon} onClick={minimize} /></button>
                    <button id='max'><img id='max-png' src={maxIcon} onClick={maximize} /></button>
                    <button id='close'><img id='close-png' src={closeIcon} onClick={close} /></button>
                </div>
            </div>
        </div>
        <div id='drag' data-tauri-drag-region></div>
        <div id="main">
            <div id="main-content">
                <div id="requests-wrapper">
                    <Table data={getData} onRowClick={onRowClick} fillHeight virtualized>
                        <Table.Column width={65}>
                            <Table.HeaderCell>Method</Table.HeaderCell>
                            <Table.Cell dataKey="method" />
                        </Table.Column>
                        <Table.Column flexGrow={2} fullText>
                            <Table.HeaderCell>Host</Table.HeaderCell>
                            <Table.Cell dataKey="host" />
                        </Table.Column>
                        <Table.Column flexGrow={2} fullText>
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
                                current ?
                                    current.protocol + "://" +
                                    current.host +
                                    current.url : ""
                            }
                        </p>
                        <p class="inspect-header">
                            <label>Request Method: </label>{current ? current.method : ""}
                        </p>
                        <Divider />
                        <p class="inspect-header-title">Request Headers</p>
                        {
                            current ?
                                Object.keys(current.headers).map(key => {
                                    return <p class="inspect-header">
                                        <label>{key}: </label>{current.headers[key]}
                                    </p>
                                }) : ""
                        }
                    </div>
                </div>
            </div>
        </div>
    </CustomProvider>
}

export default HttpSpy;