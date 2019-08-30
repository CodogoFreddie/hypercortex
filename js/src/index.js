import React from "react";
import ReactDOM from "react-dom";

import App from "./App";

export function start() {
	ReactDOM.render(<App />, document.getElementById("app"));
}

start();
