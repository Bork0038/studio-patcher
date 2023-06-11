import { useEffect, useState } from "react";
import { invoke, event, process, window as tauriWindow } from "@tauri-apps/api";
import { CustomProvider, Table, Button, Divider, Checkbox } from "rsuite";
import parseHeaders from "parse-headers";
import NetworkStream from "./classes/stream";
import { Buffer } from "buffer";

import "./RakNetSpy.css";
import "rsuite/styles/index.less";

import restoreIcon from './assets/restore.png';
import closeIcon from './assets/close.png';
import maxIcon from './assets/max.png';
import minIcon from './assets/min.png';
import icon from "./assets/icon.png";

import resumeIcon from "./assets/resume.svg";
import pauseIcon from "./assets/pause.svg";
import clearIcon from "./assets/clear.svg"
import saveIcon from "./assets/save.svg";
import openIcon from "./assets/open.svg";

import { Pc, Global } from "@rsuite/icons"

function RakNetSpy(props) {
    const [ paused, setPaused ] = useState(false);

    const [ clientHashTable, setClientHashTable ] = useState([]);
    const [ clients, setClients ] = useState([]);

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

    const addClient = ( address, packetType ) => {
        let buf = Buffer.alloc(4);
        buf.writeUint32LE( address.address.sin_addr );

        let ip = `${buf[0]}.${buf[1]}.${buf[2]}.${buf[3]}:${address.address.sin_port}`;

        const clientList = clients;
        clientList.push({
            ip,
            packetType
        });

        console.log(clientList);
        setClients( clientList );
    }

    
    useEffect(() => {
        const listen = event.listen("packet-data", (req) => {
            if (paused) return;
            const { address, opcode, packet, packet_type } = req.payload;
      
            const packetName =  Object.keys( packet )[ 0 ];
            const packetData = packet[ packetName ]; 

            const clientHash = JSON.stringify( address ) + packet_type; // hash later
            if ( !clientHashTable.includes( clientHash )) {
                addClient( address, packet_type );

                const hashTable = clientHashTable;
                hashTable.push( clientHash );
                setClientHashTable( hashTable );
            }
        });

        return () => {
            listen.then((f) => f());
        };
    });

    return <CustomProvider id="wrapper" theme="dark">
        <div id='title'>
            <p id='title-text'>RakNet Spy</p>
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
            <div id="raknet-main-content">
                <div id="toolbar-wrapper">
                    <div class="toolbar-section">
                        <div class="toolbar-main">
                            <div class="toolbar-func">
                                <div class="toolbar-button">
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="pause" src={pauseIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Pause</p>
                                </div>
                                <div class="toolbar-button">
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="resume" src={resumeIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Resume</p>
                                </div>
                            </div>
                            <p class="toolbar-text">Logging</p>
                        </div>
                        
                        <div class="toolbar-divider"></div>
                    </div>

                    <div class="toolbar-section">
                        <div class="toolbar-main">
                            <div class="toolbar-func">
                                <div class="toolbar-button">
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="clear" src={clearIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Clear</p>
                                </div>
                                <div class="toolbar-button">
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="save" src={saveIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Save</p>
                                </div>
                                <div class="toolbar-button">
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="open" src={openIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Load</p>
                                </div>
                            </div>
                            <p class="toolbar-text">Packets</p>
                        </div>
                        
                        <div class="toolbar-divider"></div>
                    </div>
                </div>

                <div id="raknet-body">
                    <div id="raknet-client-list">
                        {
                            clients.map( value => {
                                return <div class="raknet-client" onClick={() => console.log("a")}>
                                    <Checkbox />
                                    <div class="raknet-client-icon">
                                        {
                                            value.packetType == "StudioClient"
                                                ? <Pc /> 
                                                : <Global />
                                        }
                                    </div>
                                    <p class="raknet-client-address">{value.ip}</p>
                                </div>
                            })
                        }
                    </div>
                    <div id="raknet-main-body">

                    </div>
                </div>
            </div>
        </div>  
    </CustomProvider>
}

export default RakNetSpy;