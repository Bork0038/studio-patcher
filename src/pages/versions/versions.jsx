import React, { useState, useEffect } from "react";
import { Button, Table, IconButton, Form, Input, useToaster, Message, Progress } from "rsuite";
import { useNavigate } from "react-router-dom";

import { event, invoke } from "@tauri-apps/api";

import HealingIcon from '@mui/icons-material/Healing';
import EditIcon from '@mui/icons-material/Edit';
import HttpIcon from '@mui/icons-material/Http';
import TravelExploreIcon from '@mui/icons-material/TravelExplore';
import LaunchIcon from '@mui/icons-material/Launch';
import GitHubIcon from '@mui/icons-material/GitHub';
import { FaDiscord } from "react-icons/fa6";
import Plus from "@rsuite/icons/Plus";
import Icon from "../../assets/icon.png";
import Close from "@rsuite/icons/Close";

import "./versions.css";

export default function() {
    const navigate = useNavigate();

    const toaster = useToaster();
    const displayError = e => {
        toaster.push(
            <Message type="error" loca showIcon>{e.payload}</Message>,
            { placement: "topEnd" }
        );
    };

    const [ loadMessage, setLoadMessage ] = useState("");
    const [ loadPercent, setLoadPercent ] = useState(0);
    const updateProgress = d => {
        const { payload } = d;

        setLoaderHidden( false );
        setInstallHidden( true );
        setProgressHidden( false );

        setLoadMessage( payload.data );
        setLoadPercent( Math.floor(( payload.step / payload.total ) * 100) );
    }

    const updateState = () => {
        invoke( "request_state" )
            .then( d => setVersions( d.installed_versions ) )
    }
    
    const [ versions, setVersions ] = useState([]);
    useEffect(() => {
        updateState()

        const error_listener = event.listen( "install_version_error", displayError );
        const progress_listener = event.listen( "install_version_progress", updateProgress );
        const finished_listener = event.listen( "install_version_done", () => {
            updateState()

            setLoaderHidden( true );
            setInstallHidden( true );
            setProgressHidden( true );    
        })

        return () => {
            error_listener.then( f => f() );
            progress_listener.then( f => f() );
            finished_listener.then( f => f() );
        };
    })

    const [ loaderHidden, setLoaderHidden ] = useState(true);
    const [ installHidden, setInstallHidden ] = useState(true);
    const [ progressHidden, setProgressHidden ] = useState(true);

    const openInstallModal = () => {
        setInstallHidden( false );
        setLoaderHidden( false );
    };

    const closeInstallModal = () => {
        setInstallHidden( true );
        setLoaderHidden( true );
    };

    const installVersion = () => {
        const channel = document.getElementById("version-channel").value;
        const version = document.getElementById("version-version").value;

        invoke(
            "install_version",
            {
                req: {
                    channel,
                    version
                }
            }
        )
    };

    return <div class="main">
        <div class="sidebar">
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <img src={Icon} />
                    <p>Placeholder</p>
                </div>
            </div>
            <p class="sidebar-title">TOOLS</p>
            <Button onClick={ () => navigate("/") } className="sidebar-button">
                <div class="sidebar-button-inner">
                    <HealingIcon />
                    <p>Patches</p>
                </div>
            </Button>
            <Button onClick={ () => {} } className="sidebar-button sidebar-selected">
                <div class="sidebar-button-inner">
                    <LaunchIcon />
                    <p>Version Manager</p>
                </div>
            </Button>
            <Button className="sidebar-button">
                <div class="sidebar-button-inner">
                    <EditIcon />
                    <p>Fast Variable Editor</p>
                </div>
            </Button>
            <Button className="sidebar-button">
                <div class="sidebar-button-inner">
                    <HttpIcon />
                    <p>HTTP Spy</p>
                </div>
            </Button>
            <Button className="sidebar-button">
                <div class="sidebar-button-inner">
                    <TravelExploreIcon />
                    <p>RakNet Spy</p>
                </div>
            </Button>
            <div class="sidebar-divider" />
            <p class="sidebar-title">LINKS</p>
            <Button className="sidebar-button">
                <div class="sidebar-button-inner">
                    <GitHubIcon />
                    <p>GitHub</p>
                </div>
            </Button>
            <Button className="sidebar-button">
                <div class="sidebar-button-inner">
                    <FaDiscord />
                    <p>Discord</p>
                </div>
            </Button>
            <p class="sidebar-footer">version 1.0.0</p>
        </div>
        <div class="main-page">
            <div class="loader" hidden={loaderHidden}>
                <div class="loader-inner card" hidden={installHidden}>
                    <div class="versions-title-wrapper">
                        <p class="card-title versions-title">Install Version</p> 
                        <IconButton onClick={closeInstallModal} icon={<Close />}     />
                    </div>
                    <Form id="content" layout="vertical" fluid>
						<Form.Group controlId="version-channel">
							<Form.ControlLabel>Channel</Form.ControlLabel>
							<Input />
						</Form.Group>
                        <Form.Group controlId="version-version">
							<Form.ControlLabel>Version</Form.ControlLabel>
							<Input />
						</Form.Group>
						<Form.Group id="install-group">
							<Button color="primary" onClick={installVersion}>Install</Button>
						</Form.Group>
					</Form>
                </div>
                <div class="loader-inner card" id="loader-progress" hidden={progressHidden}>
                    <p class="card-title versions-title">{loadMessage}</p> 
                    <Progress.Line percent={loadPercent}/>
                </div>
            </div>
            <div class="main-page-header">
                <p class="main-page-header-title">Version Manager</p>
            </div>
            <div class="main-page-content">
                <div class="versions-card card">
                    <div class="versions-title-wrapper">
                        <p class="card-title versions-title">Versions</p> 
                        <IconButton onClick={openInstallModal} icon={<Plus />}     />
                    </div>
                    <Table 
                        className="versions-table"
                        data={versions}
                    >
                        <Table.Column flexGrow={1}>
                            <Table.HeaderCell>Channel</Table.HeaderCell>
                            <Table.Cell dataKey="channel" />
                        </Table.Column>
                        <Table.Column flexGrow={1}>
                            <Table.HeaderCell>Version</Table.HeaderCell>
                            <Table.Cell dataKey="version" />
                        </Table.Column>
                        <Table.Column flexGrow={0.5}>
                            <Table.HeaderCell></Table.HeaderCell>
                            <Table.Cell style={{padding: 0}}>
                                {
                                    data => (
                                        <Button className="versions-table-button">Launch</Button>
                                    )
                                }
                            </Table.Cell>
                        </Table.Column>
                        <Table.Column flexGrow={0.5} >
                            <Table.HeaderCell></Table.HeaderCell>
                            <Table.Cell style={{padding: 0}}>
                                {
                                    data => (
                                        <Button className="versions-table-button versions-table-delete">Delete</Button>
                                    )
                                }
                            </Table.Cell>
                        </Table.Column>
                    </Table>  
                </div>
            </div>
        </div>
        <div class="main-drag" data-tauri-drag-region/>
    </div>
}