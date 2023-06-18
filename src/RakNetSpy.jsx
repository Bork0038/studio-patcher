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

import { Pc, Global, SortUp, SortDown } from "@rsuite/icons"

function RakNetSpy(props) {
    const [ paused, setPaused ] = useState(false);

    const [ clientHashTable, setClientHashTable ] = useState([]);
    const [ clients, setClients ] = useState([]);
    const [ enabledClients, setEnabledClients ] = useState({});

    const [ packets, setPackets ] = useState([]);
    const [ activePacket, setActivePacket ] = useState([]);

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

    const getIpFromAddress = address => {
        let buf = Buffer.alloc(4);
        buf.writeUint32LE( address.address.sin_addr );

        return `${buf[0]}.${buf[1]}.${buf[2]}.${buf[3]}:${address.address.sin_port}`;
    }

    const addClient = ( address, packetType, clientHash ) => {
        const clientList = [ ...clients ];
        clientList.push({
            ip: getIpFromAddress( address ),
            packetType,
            hash: clientHash
        });

        setClients( clientList );
    }

    const toggleClient = clientHash => {
        setEnabledClients({
            [clientHash]: !enabledClients[ clientHash ]
        })
    }

    const bytesFromInt = int => {
        return [
            (int >> 8) & 0xFF,
            int & 0xFF
        ]
    }
    
    const getPackets = () => {
        const clientHashes = Object
            .keys( enabledClients)
            .filter( key => enabledClients[ key ] );

        return packets.filter( packet => !clientHashes.includes( packet.clientHash ) );
    }

    const onRowClick = row => {
        if (paused) return;

        let { packet } = row;

        let validKeys = Object
            .keys( packet )
            .filter( 
                key => 
                    key != "id" && 
                    key != "len" &&
                    packet[ key ] != null
            )

        const newActivePacket = [
            <div class="raknet-inspector-pair">
                <p class="raknet-inspector-pair-key">ID</p>
                <p class="raknet-inspector-pair-value">{ row.id }</p>
            </div>,
             <div class="raknet-inspector-pair">
                <p class="raknet-inspector-pair-key">Name</p>
                <p class="raknet-inspector-pair-value">{ row.name }</p>
            </div>,
            <div class="raknet-inspector-pair">
                <p class="raknet-inspector-pair-key">Source</p>
                <p class="raknet-inspector-pair-value">{ row.client }</p>
            </div>,
            <div class="raknet-inspector-pair">
                <p class="raknet-inspector-pair-key">Type</p>
                <p class="raknet-inspector-pair-value">{ row.packetType }</p>
            </div>,

            <p class="raknet-inspector-title">Packet Data</p>
        ];
        for (let key of validKeys) {
            let value = packet[key];
            
            if (typeof value == "object") {
                newActivePacket.push(
                    <p class="raknet-inspector-title">{ key }</p>
                );

                for ( let key of Object.keys( value ) ) {
                    let value2 = value[ key ];

                    if (typeof value2 == "object" && value2.length == 2) {
                        newActivePacket.push(
                            <div class="raknet-inspector-pair">
                                <p class="raknet-inspector-pair-key">{ value2[ 0 ] }</p>
                                <p class="raknet-inspector-pair-value">{ value2[ 1 ] }</p>
                            </div>
                        );
                    } else {
                        newActivePacket.push(
                            <div class="raknet-inspector-pair">
                                <p class="raknet-inspector-pair-key">{ key }</p>
                                <p class="raknet-inspector-pair-value">{ value2.toString() }</p>
                            </div>
                        );
                    }
                }
            } else {
                newActivePacket.push(
                    <div class="raknet-inspector-pair">
                        <p class="raknet-inspector-pair-key">{ key }</p>
                        <p class="raknet-inspector-pair-value">{ value.toString() }</p>
                    </div>
                );
            }
        }

        setActivePacket( newActivePacket );
    }

    const clearPackets = () => {
        setClients([]);
        setPackets([]);
        setActivePacket([]);
        setClientHashTable([]);
        setEnabledClients({});
    }

    const pause = () => {
        setPaused(true);
    }

    const unpause = () => {
        setPaused(false);
    }

    useEffect(() => {
        const listen = event.listen("packet-data", (req) => {
            if (paused) return;

            const { address, opcode, packet, packet_type } = req.payload;

            const packetName =  Object.keys( packet )[ 0 ];
            const packetData = packet[ packetName ]; 


            const clientHash = JSON.stringify( address ) + packet_type; // hash later
            if ( !clientHashTable.includes( clientHash )) {
                addClient( address, packet_type, clientHash );

                const hashTable = clientHashTable;
                hashTable.push( clientHash );
                setClientHashTable( hashTable );
            }

            let id_bytes = bytesFromInt( packetData.id );
            let id_string = id_bytes[ 0 ] == 0x83 
                ? `${id_bytes[ 0 ].toString( 16 )} ${id_bytes[ 1 ].toString( 16 )}`
                : id_bytes[ 1 ].toString( 16 );

            const packetList = [ ...packets ];
            packetList.push({
                id: id_string,
                name: packetName,
                client: getIpFromAddress( address ),
                len: packetData.len,
                opcode,
                packet: packetData,
                packetType: packet_type,
                clientHash
            })
            
            setPackets( packetList );
        });

        return () => {
            listen.then(f => f());
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
                                <div class="toolbar-button" onClick={pause}>
                                    <div class="toolbar-button-icon">
                                        <img class="toolbar-button-icon-image" id="pause" src={pauseIcon}></img> {/* thank you convoluted name #1 */}
                                    </div>
                                    <p class="toolbar-button-text">Pause</p>
                                </div>
                                <div class="toolbar-button" onClick={unpause}>
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
                                <div class="toolbar-button" onClick={clearPackets}>
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
                                return <div class="raknet-client" onClick={ () => toggleClient( value.hash ) }>
                                    <Checkbox checked={ !enabledClients[ value.hash ] } />
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
                        <div id="raknet-packet-list">
                            <Table data={getPackets} onRowClick={onRowClick} shouldUpdateScroll={()=>true} fillHeight virtualized>
                                <Table.Column width={65} fullText>
                                    <Table.HeaderCell></Table.HeaderCell>
                                    <Table.Cell>
                                        {
                                            rowData => <div class="raknet-arrow-cell">
                                                <div class="raknet-arrow-inner">
                                                    { 
                                                        rowData.opcode == "OutgoingPackets" 
                                                            ? <SortUp class="sort-up" />
                                                            : <SortDown class="sort-down" />
                                                    }
                                                </div>
                                            </div>
                                        }
                                    </Table.Cell>
                                </Table.Column>
                                <Table.Column width={65}>
                                    <Table.HeaderCell>ID</Table.HeaderCell>
                                    <Table.Cell dataKey="id" />
                                </Table.Column>
                                <Table.Column flexGrow={2} fullText>
                                    <Table.HeaderCell>Name</Table.HeaderCell>
                                    <Table.Cell dataKey="name" />
                                </Table.Column>
                                <Table.Column flexGrow={1} fullText>
                                    <Table.HeaderCell>Source</Table.HeaderCell>
                                    <Table.Cell dataKey="client" />
                                </Table.Column>
                                <Table.Column flexGrow={0.5} fullText>
                                    <Table.HeaderCell># of Bytes</Table.HeaderCell>
                                    <Table.Cell dataKey="len" />
                                </Table.Column>
                            </Table>
                        </div>
                        <div id="raknet-packet-inspector">
                            <p class="raknet-inspector-title">Packet Info</p>
                            {
                                activePacket
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>  
    </CustomProvider>
}

export default RakNetSpy;