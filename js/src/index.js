import { run } from "../../rust/hypertask_npm_package/Cargo.toml";
import React from "react";
import ReactDOM from "react-dom";

import App from "./App";

export function start() {
	ReactDOM.render(<App />, document.getElementById("app"));
}

start();