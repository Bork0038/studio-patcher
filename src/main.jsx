import React from "react";
import ReactDOM from "react-dom/client";
import {
	createBrowserRouter,
	RouterProvider
} from "react-router-dom"

import App from "./Home";
import HttpSpy from "./HttpSpy";

const router = createBrowserRouter([
	{
		path: "/",
		element: <App />
	},
	{
		path: "/http",
		element: <HttpSpy />
	}
])


ReactDOM.createRoot(document.getElementById("root")).render(
	<React.StrictMode>
		<RouterProvider router={ router } />
	</React.StrictMode>
);
