import { useEffect, useState } from "react";
import { invoke, event, process, window as tauriWindow } from "@tauri-apps/api";
import { CustomProvider, Table, Button, Divider } from "rsuite";
import parseHeaders from "parse-headers";
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
    const [bodyShown, setBodyShown] = useState(false);
    const [body, setBody] = useState("");

    const onRowClick = (row) => {
        hideBody();
        setCurrent(row);
    };

    const getData = () => {
        return data;
    }

    const close = async () => {
		await tauriWindow
			.getCurrent()
            .close();
	}

	const minimize = async () => {
		await tauriWindow
			.getCurrent()
			.minimize();
	}

	const maximize = async () => {
		const currentWindow = tauriWindow.getCurrent();
		const isMaximized   = await currentWindow.isMaximized();

		document.getElementById('max-png').src = isMaximized ? maxIcon : restoreIcon;
		isMaximized ? currentWindow.unmaximize() : currentWindow.maximize();

		this.setState();
	}

    const setCurrentBody = (key) => {
        if (current) {
            const headers = current[key + "Headers"];
            const contentType = headers["content-type"] ?? headers["Content-Type"] ?? "";

            let data = current[key + "Body"];
            if (contentType.includes("application/json")) {
                try {
                    data = JSON.stringify(
                        JSON.parse(data),
                        null,
                        4
                    );
                } catch(e) {}
            }

            setBody(data);
            setBodyShown(true)
        }
    }

    const hideBody = () => {
        setBody("");
        setBodyShown(false)
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
                statusCode: stream.readInt32LE(),
                resHeaders: stream.readString32(),
                resBody: stream.readString32(),
                url: stream.readString32(),
                method: stream.readString32(),
                reqBody: stream.readString32(),
            };

            let num_headers = stream.readInt32LE();
            let headers = {};
            for (let header = 0; header < num_headers; header++) {
                let key = stream.readString32();
                let value = stream.readString32();

                headers[key] = value;
            }
            packet.reqHeaders = headers;
            packet.resHeaders = parseHeaders(packet.resHeaders)

            let req_data = [...data];
            req_data.push(packet);
            console.log(packet);
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
                    <button id='min' onClick={minimize}><img id='min-png' src={minIcon} /></button>
                    <button id='max' onClick={maximize}><img id='max-png' src={maxIcon} /></button>
                    <button id='close' onClick={close}><img id='close-png' src={closeIcon} /></button>
                </div>
            </div>
        </div>
        <div id='drag' data-tauri-drag-region></div>
        <div id="main">
            <div id="main-content">
                <div id="requests-wrapper">
                    <Table data={getData} onRowClick={onRowClick} shouldUpdateScroll={()=>true} fillHeight virtualized>
                    <Table.Column width={65} fullText>
                            <Table.HeaderCell>Status</Table.HeaderCell>
                            <Table.Cell dataKey="statusCode" />
                        </Table.Column>
                        <Table.Column width={65}>
                            <Table.HeaderCell>Method</Table.HeaderCell>
                            <Table.Cell dataKey="method" />
                        </Table.Column>
                        <Table.Column flexGrow={2} fullText>
                            <Table.HeaderCell>Url</Table.HeaderCell>
                            <Table.Cell dataKey="url" />
                        </Table.Column>
                    </Table>
                </div>
                <div id="inspect-wrapper">
                    <div id="inspect-type">
                        <Button class="inspect-button" onClick={hideBody}>Headers</Button>
                        <Button class="inspect-button" onClick={()=>setCurrentBody("req")}>Request Body</Button>
                        <Button class="inspect-button" onClick={()=>setCurrentBody("res")}>Response Body</Button>
                    </div>
                    <div id="inspect-headers" hidden={bodyShown}>
                        <p class="inspect-header-title">General</p>
                        <p class="inspect-header">
                            <label>Request URL: </label>{
                                current ? current.url : ""
                            }
                        </p>
                        <p class="inspect-header">
                            <label>Request Method: </label>{current ? current.method : ""}
                        </p>
                        <Divider />
                        <p class="inspect-header-title">Request Headers</p>
                        {
                            current ?
                                Object.keys(current.reqHeaders).map(key => {
                                    return <p class="inspect-header">
                                        <label>{key}: </label>{current.reqHeaders[key]}
                                    </p>
                                }) : ""
                        }
                        <Divider />
                        <p class="inspect-header-title">Response Headers</p>
                        {
                            current ?
                                Object.keys(current.resHeaders).map(key => {
                                    return <p class="inspect-header">
                                        <label>{key}: </label>{current.resHeaders[key]}
                                    </p>
                                }) : ""
                        }
                    </div>
                    <div id="inspect-body" hidden={!bodyShown}>
                        <pre>
                            {
                                bodyShown ? body : ""
                            }
                        </pre>
                    </div>
                </div>
            </div>
        </div>
    </CustomProvider>
}

export default HttpSpy;