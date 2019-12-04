import React from "react";
import cs from "classname";

import css from "./styles.scss";

export default function EditorOutput({ final, minifiedProgram, traceError }) {
	if (traceError) {
		return <output>error: {traceError}</output>;
	} else {
		return <output>program: {minifiedProgram}</output>;
	}
}
