import React from "react";

export default function EditorOutput({ minifiedProgram, traceError }) {
	if (traceError) {
		return <output>error: {traceError}</output>;
	} else {
		return <output>program: {minifiedProgram}</output>;
	}
}
