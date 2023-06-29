import React, { useState, useEffect } from "react";
import { CustomProvider, Tree, InputGroup, Input, Table } from "rsuite";

import { window as tauriWindow, event } from "@tauri-apps/api";

import "./SchemaViewer.css";
import "rsuite/styles/index.less";

import Search from '@rsuite/icons/Search';
import FolderFillIcon from '@rsuite/icons/FolderFill';
import restoreIcon from './assets/restore.png';
import closeIcon from './assets/close.png';
import maxIcon from './assets/max.png';
import minIcon from './assets/min.png';
import icon from "./assets/icon.png";

function App() {
    const [ schema, setSchema ] = useState([]);
    const [ currentClass, setCurrentClass ] = useState(null);
    const [ currentEvents, setCurrentEvents ] = useState([]);

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
	}

    useEffect(() => {
        event.listen("init-schema", (e) => {
            const tempSchema = e.payload;
            const newSchema = [
                {
                    label: "Enums",
                    value: tempSchema.enums,
                },
                {
                    label: "Optimized Strings",
                    value: tempSchema.optimized_strings,
                },
                {
                    label: "Content Prefixes",
                    value: tempSchema.content_prefixes,
                },
                {
                    label: "Classes",
                    value: tempSchema.classes,
                    children: tempSchema.classes.map(e => {
                        return { 
                            label: e.name,
                            value: { ...e, type: "class" }
                        }
                    })
                },
            ];

            setSchema( newSchema );
        })
    });

    const onSelect = element => {
        const data = element.value;

        if (data.type == "class") {
            const events = data.events.map(d => {
                return {
                    label: d.name,
                    value: d
                }
            })

            setCurrentEvents( events );
            setCurrentClass( data );
        }
    }

    const [ searchTerm, setSearchTerm ] = useState(null);
    const search = () => {
		let value = document.getElementById("search-input").value;

		setSearchTerm( value.length == 0 ? null : value );
	}

    const [ currentEvent, setCurrentEvent ] = useState(null);
    const onSelectEvent = event => {
        setCurrentEvent( event.value );
    }

    return (
        <CustomProvider id="wrapper" theme="dark">
            <div id='title'>
                <p id='title-text'>Schema Viewer</p>
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
                <div id='drag' data-tauri-drag-region></div>
            </div>
            <div id="main">
                <div id="schema-sidebar">
                    <p class="sidebar-panel">Search</p>
                    <InputGroup id="search">
                        <Input id="search-input" onChange={search} />
                        <InputGroup.Button onClick={search}>
                            <Search />
                        </InputGroup.Button>
                    </InputGroup>
                    <p class="sidebar-panel">Schema</p>
                    <Tree 
                        data={schema}
                        renderTreeNode={node => {
                            return (
                              <>
                                {node.children ? <FolderFillIcon /> : ""} {node.label}
                              </>
                            );
                        }}
                        searchKeyword={searchTerm}
                        onSelect={onSelect}
                    />
                </div>
                <div class="schema-main">
                    <div id="schema-class-viewer">
                        <p class="raknet-inspector-title">{ currentClass ? currentClass.name : ""}</p>
                        <div class="schema-class-info">
                            <div class="schema-class-pair">
                                <p class="schema-class-pair-key">Network ID</p>
                                <p class="schema-class-pair-value">{ currentClass ? currentClass.network_id : "" }</p>
                            </div>
                        </div>
                        <p class="raknet-inspector-title">Properties</p>
                        <div class="schema-class-properties">
                            <Table data={() => currentClass ? currentClass.properties : []} fillHeight virtualized>
                                <Table.Column flexGrow={0.5} fullText>
                                    <Table.HeaderCell>Name</Table.HeaderCell>
                                    <Table.Cell dataKey="name" />
                                </Table.Column>
                                <Table.Column flexGrow={0.5} fullText>
                                    <Table.HeaderCell>Type</Table.HeaderCell>
                                    <Table.Cell dataKey="prop_type" />
                                </Table.Column>
                                <Table.Column flexGrow={0.25} fullText>
                                    <Table.HeaderCell>Network ID</Table.HeaderCell>
                                    <Table.Cell dataKey="network_id" />
                                </Table.Column>
                                <Table.Column flexGrow={0.25} fullText>
                                    <Table.HeaderCell>Enum ID</Table.HeaderCell>
                                    <Table.Cell dataKey="prop_enum_id" />
                                </Table.Column>
                            </Table>
                        </div>
                        <p class="raknet-inspector-title">Events</p>
                        <div class="schema-class-events">
                            <div class="schema-class-events-sidebar">
                                <p class="raknet-inspector-title">Name</p>
                                <div class="schema-class-events-sidebar-inner">
                                    <Tree 
                                        data={currentEvents}
                                        onSelect={onSelectEvent}
                                    />
                                </div>
                            </div>
                            <div class="schema-class-events-main">
                                <p class="raknet-inspector-title">Arguments</p>
                                <Table data={() => currentEvent ? currentEvent.arguments : []} fillHeight virtualized>
                                    <Table.Column flexGrow={0.5} fullText>
                                        <Table.HeaderCell>Type</Table.HeaderCell>
                                        <Table.Cell dataKey="argument_type" />
                                    </Table.Column>
                                    <Table.Column flexGrow={0.5} fullText>
                                        <Table.HeaderCell>Enum ID</Table.HeaderCell>
                                        <Table.Cell dataKey="argument_enum_id" />
                                    </Table.Column>
                                </Table>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </CustomProvider>
    );
}

export default App;
