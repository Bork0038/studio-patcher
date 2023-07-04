import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { CustomProvider } from "rsuite";

import Patches from "./pages/patches/patches";
import Versions from "./pages/versions/versions";

import "./rsuite.less";
import "./main.css";

const router = createBrowserRouter([
	{
		path: "/",
		element: <Patches />
	},
	{
		path: "/versions",
		element: <Versions />
	}
])


ReactDOM.createRoot(document.getElementById("root")).render(
	<React.StrictMode>
		<CustomProvider id="wrapper" theme="dark">
			<RouterProvider router={ router } />
		</CustomProvider>
	</React.StrictMode>
);
