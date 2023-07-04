import React, { useState } from "react";
import { Button, Table } from "rsuite";
import { useNavigate } from "react-router-dom";

import HealingIcon from '@mui/icons-material/Healing';
import EditIcon from '@mui/icons-material/Edit';
import HttpIcon from '@mui/icons-material/Http';
import TravelExploreIcon from '@mui/icons-material/TravelExplore';
import LaunchIcon from '@mui/icons-material/Launch';
import GitHubIcon from '@mui/icons-material/GitHub';
import { FaDiscord } from "react-icons/fa6";
import Icon from "../../assets/icon.png";

import "./versions.css";

export default function() {
    const navigate = useNavigate();

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
            <div class="main-page-header">
                <p class="main-page-header-title">Version Manager</p>
            </div>
            <div class="main-page-content">
                <div class="versions-card card">
                    <p class="card-title">Versions</p> 
                    <Table 
                        className="versions-table"
                        data={[
                            {
                                channel: "zlive",
                                version: "version-3a04a239b8424d46"
                            }
                        ]}
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