import React, { useState } from "react";
import { Button, RadioTile, Form, InputGroup } from "rsuite";
import { useNavigate } from "react-router-dom";

import HealingIcon from '@mui/icons-material/Healing';
import EditIcon from '@mui/icons-material/Edit';
import HttpIcon from '@mui/icons-material/Http';
import TravelExploreIcon from '@mui/icons-material/TravelExplore';
import LaunchIcon from '@mui/icons-material/Launch';
import GitHubIcon from '@mui/icons-material/GitHub';
import Search from "@rsuite/icons/Search";
import Global from "@rsuite/icons/Global";
import Shield from "@rsuite/icons/Shield";
import ExploreIcon from '@rsuite/icons/Explore';
import PaletteIcon from '@mui/icons-material/Palette';
import CloudOffIcon from '@mui/icons-material/CloudOff';

import { FaDiscord } from "react-icons/fa6";

import "./patches.css";
import Icon from "../../assets/icon.png";

export default function() {
    const patches = [
        {
            name: "internal-studio",
            icon: <Shield />,
            title: "Internal Studio",
            description: "Enables Roblox's Internal Studio mode. Gives access to features such as FFlag editor, additional plugins...",
        },
        {
            name: "extended-explorer",
            icon: <ExploreIcon />,
            title: "Extended Explorer",
            description: "Shows hidden properties and instances in the Studio explorer",
        },
        {
            name: "http-spy",
            icon: <HttpIcon />,
            title: "HTTP Spy",
            description: "Logs HTTP requests made by Studio",
        },
        {
        	name: "themes",
            icon: <PaletteIcon />,
        	title: "Themes",
        	description: "Adds more themes to studio"
        },
        {
        	name: "disable-telemetry",
            icon: <CloudOffIcon />,
        	title: "Disable Telemetry",
        	description: "It disables telemetry "
        }
    ];

    const navigate = useNavigate();

    const [ activePatches, setActivePatches ] = useState({});
    const activatePatch = patch => {
        setActivePatches(old => {
            const newPatches = { ...old };
            newPatches[ patch.title ] = !newPatches[ patch.title ];
            
            return newPatches;
        })
    }

    return <div class="main">
        <div class="sidebar">
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <img src={Icon} />
                    <p>Placeholder</p>
                </div>
            </div>
            <p class="sidebar-title">TOOLS</p>
            <Button onClick={() => {}} className="sidebar-button sidebar-selected">
                <div class="sidebar-button-inner">
                    <HealingIcon />
                    <p>Patches</p>
                </div>
            </Button>
            <Button onClick={ () => navigate("/versions") } className="sidebar-button">
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
                <p class="main-page-header-title">Patches</p>
            </div>
            <div class="main-page-content">
                <div class="patches-patches card">
                    <p class="card-title">Patches</p>
                    <div class="patch-holder">
                        {
                            patches.map(d => {
                                return <RadioTile 
                                    icon={d.icon}
                                    label={d.title}
                                    checked={activePatches[d.title]}
                                    onClick={() => activatePatch(d)}
                                >
                                    {d.description}
                                </RadioTile>
                            })
                        }
                    </div>
                </div>
                <div class="patches-options card" >
                    <p class="card-title">Install Patches</p>
                    <Form id="content" layout="vertical" fluid>
						<Form.Group controlId="version" id="version-group">
							<Form.ControlLabel>Studio Executable</Form.ControlLabel>
							<InputGroup>
								<InputGroup.Addon>
									<Search />
								</InputGroup.Addon>
								<Form.Control name="version" readOnly value={""}/>
								<Button id="select" onClick={() =>{}}>Select</Button>
							</InputGroup>
						</Form.Group>
						<Form.Group id="install-group">
							<Button color="primary" onClick={() =>{}}>Install</Button>
						</Form.Group>
					</Form>
                </div>
            </div>
        </div>
        <div class="main-drag" data-tauri-drag-region/>
    </div>
}