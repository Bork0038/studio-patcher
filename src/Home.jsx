import React, { Component } from "react";
import { 
	CustomProvider, 
	Form, 
	InputGroup, 
	Button, 
	Panel, 
	Checkbox,
	Message,
	toaster,
	Divider,
	Loader
} from "rsuite";

import { Search, Global } from "@rsuite/icons";
import { dialog, invoke, process, window, event } from "@tauri-apps/api";
import { WebviewWindow } from '@tauri-apps/api/window'

import "./Home.css";
import "rsuite/styles/index.less";

import restoreIcon from './assets/restore.png';
import closeIcon from './assets/close.png';
import maxIcon from './assets/max.png';
import minIcon from './assets/min.png';
import icon from "./assets/icon.png";

class App extends Component {
	
	constructor(props) {
		super(props);

		this.state = {
			robloxPath: "",
			version: "",
			patches: [
				// {
				// 	name: "internal-studio",
				// 	title: "Internal Studio",
				// 	description: "Enables Roblox's Internal Studio mode. Gives access to features such as FFlag editor, additional plugins...",
				// },
				// {
				// 	name: "extended-explorer",
				// 	title: "Extended Explorer",
				// 	description: "Shows hidden properties and instances in the Studio explorer",
				// },
				{
					name: "themes",
					title: "Themes",
					description: "Adds more themes to studio"
				},
				// {
				// 	name: "disable-telemetry",
				// 	title: "Disable Telemetry",
				// 	description: "It disables telemetry "
				// }
			],
			refs: {
				tools: React.createRef()
			},
		}

        this.handleClick     = this.handleClick.bind(this);
		this.openWindow		 = this.openWindow.bind(this);
		this.openTab         = this.openTab.bind(this);
		this.onHover         = this.onHover.bind(this);
		this.openFileDialog = this.openFileDialog.bind( this );
		this.restoreBinary  = this.restoreBinary.bind(this);
		this.maximize  		= this.maximize.bind( this );
		this.minimize		= this.minimize.bind( this );
		this.submit			= this.submit.bind( this );
		this.close 			= this.close.bind( this );
	}

	async submit() {
		if ( this.state.robloxPath == "" ) {
			return toaster.push(
				<Message showIcon type="error">
					Executable path cannot be empty
				</Message>
			)
		}

		const loader = document.getElementById("loading-screen");
		const form   = document.forms[0];

		const patches = [];
		for ( let patch of this.state.patches ) {
			if ( form[patch.name].checked ) {
				patches.push( patch.name );
			}
		}

		event.once( "installed_patches", eventData => {
			const { payload } = eventData;

			loader.style.visibility = "hidden";
			if ( !payload.success ) {
				return toaster.push(
					<Message showIcon type="error">
						Failed to patch studio { payload.data }
					</Message>
				)
			} else {
				return toaster.push(
					<Message showIcon type="success">
						Successfully patched studio
					</Message>
				)
			}
		})

		loader.style.visibility = "visible";
		await invoke(
			"install_patches",
			{
				patches: {
					path: this.state.robloxPath,
					patches,
				}
			}
		)
	}

	async restoreBinary() {
		if ( this.state.robloxPath == "" ) {
			return toaster.push(
				<Message showIcon type="error">
					Executable path cannot be empty
				</Message>
			)
		}

		const loader = document.getElementById("loading-screen");
		event.once( "restored_version", eventData => {
			const { payload } = eventData;

			loader.style.visibility = "hidden";
			if ( !payload.success ) {
				return toaster.push(
					<Message showIcon type="error">
						Failed to restore studio { payload.data }
					</Message>
				)
			} else {
				return toaster.push(
					<Message showIcon type="success">
						Successfully restored studio
					</Message>
				)
			}
		})

		loader.style.visibility = "visible";
		await invoke(
			"restore_version",
			{
				patches: {
					path: this.state.robloxPath,
					version: document.getElementById("restore-version").value,
				}
			}
		)
	}

	async openFileDialog() {
		const data = await dialog.open({
			filters: [{
				name: "RobloxStudioBeta",
				extensions: [ "exe" ]
			}]
		});

		this.setState({
			robloxPath: data
		});
	}

	async close() {
		await process.exit();
	}

	async minimize() {
		await window
			.getCurrent()
			.minimize();
	}

	async maximize() {
		const currentWindow = window.getCurrent();
		const isMaximized   = await currentWindow.isMaximized();

		document.getElementById('max-png').src = isMaximized ? maxIcon : restoreIcon;
		isMaximized ? currentWindow.unmaximize() : currentWindow.maximize();

		this.setState();
	}

	componentDidMount() {
		this.setState({
			tabs: {
				tools: document.getElementById('tools-tab'),
			}
		})

		document.addEventListener('mousedown', this.handleClick);
	}

	openTab(tab) {
		for (let tabName in this.state.tabs) {
			let entry = this.state.tabs[tabName];
			
			entry.style.visibility = tabName == tab ? 'visible' : 'hidden';

			if (tabName == tab) {
				if (this.state.openTab == entry) {
					entry.style.visibility = 'hidden';
					this.setState({
						openTab: null,
					})
					this.openRef = null;
				} else {
					this.setState({
						openTab: entry
					})
					this.openRef = this.state.refs[tabName];
				}
			}
		}
    }

	openWindow(location) {
		const webview = new WebviewWindow( location, {
			url: location,
			decorations: false	
		});
		webview.show();
	}

	handleClick(event) {
		if (this.openRef && !this.openRef.current.contains(event.target)) {
			this.state.openTab.style.visibility = 'hidden';
			this.setState({
				openTab: null
			})
			this.openRef = null;
		}
	}

    onHover(tab) {
		if (this.state.openTab && this.state.tabs[tab] != this.state.openTab) {
			this.openTab(tab);
		}
	}


	render() {
		return (
			<CustomProvider id="wrapper" theme="dark">
				<div id="tabs">
					<div id='tools-tab' ref={this.state.refs.tools}>
						<button class='tab-entry' onClick={() => this.openWindow("/http")}>
							<p class='tab-title'>HTTP Spy</p>
						</button>
						<button class='tab-entry' onClick={() => this.openWindow("/")}>
							<p class='tab-title'>RakNet Spy</p>
						</button>
					</div>
				</div>
				<div id='title'>
					<p id='title-text'>Studio Patcher</p>
					<div id='title-left'>
						<div id='icon-wrapper'>
							<img id='icon' src={icon} />
						</div>
						<div id='navigation-wrapper'>
							<button id='tools' class='button-left' onMouseOver={() => this.onHover('tools')} onClick={() => this.openTab('tools')}>Tools</button>
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
				<div id="loading-screen">
					<div id="loading-screen-inner">
						<p>Applying Patches...</p>
						<Loader center size="lg" id="loading-screen-loader" />
					</div>
				</div>
				<div id='drag' data-tauri-drag-region></div>
				<div id="main">
					<Form id="content" layout="vertical" fluid>
						<Form.Group controlId="version" id="version-group">
							<Form.ControlLabel>Studio Executable</Form.ControlLabel>
							<InputGroup>
								<InputGroup.Addon>
									<Search />
								</InputGroup.Addon>
								<Form.Control name="version" readOnly value={ this.state.robloxPath }/>
								<Button id="select" onClick={this.openFileDialog}>Select</Button>
							</InputGroup>
						</Form.Group>
						<Form.Group controlId="patches" id="patches-group">
							<Form.ControlLabel>Patches</Form.ControlLabel>
							<Panel bordered bodyFill shaded id="patches-panel">
								<div id="patches-wrapper">
									{
										this.state.patches.map(patch => {
											return <div class="patch">
												<Checkbox name={patch.name}>{patch.title}</Checkbox>
												<p class="patch-description">{patch.description}</p>
												<Divider class="patch-divider"/>
											</div>
										})
									}

								</div>
								
							</Panel>
						</Form.Group>
						<Form.Group id="install-group">
							<Button onClick={this.submit}>Install Patches</Button>
						</Form.Group>
						<Form.Group id="remove-group">
							<Form.ControlLabel>Remove Patches</Form.ControlLabel>
							<InputGroup>
								<InputGroup.Addon>
									<Global />
								</InputGroup.Addon>
								<Form.Control id="restore-version" placeholder="Version"></Form.Control>
								<Button id="restore" onClick={this.restoreBinary}>Restore Executable</Button>
							</InputGroup>
						</Form.Group>
					</Form>
				</div>
			</CustomProvider>
		);
	}
}

export default App;
