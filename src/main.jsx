import React from "react";
import ReactDOM from "react-dom/client";
import {
	createBrowserRouter,
	RouterProvider
} from "react-router-dom"

import App from "./Home";
import HttpSpy from "./HttpSpy";
import RakNetSpy from "./RakNetSpy";
import SchemaViewer from "./SchemaViewer";

const router = createBrowserRouter([
	{
		path: "/",
		element: <App />
	},
	{
		path: "/http",
		element: <HttpSpy />
	},
	{
		path: "/raknet",
		element: <RakNetSpy />
	},
	{
		path: "/schema",
		element: <SchemaViewer />
	}
])


ReactDOM.createRoot(document.getElementById("root")).render(
	<React.StrictMode>
		<RouterProvider router={ router } />
	</React.StrictMode>
);
