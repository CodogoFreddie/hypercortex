import React from "react";
import cs from "classname";

import Labeled from "../Labeled";

import css from "./styles.scss";

export default function EditorOutput({ trace, minifiedProgram, traceError }) {
	const final = React.useMemo(() => (trace.slice(-1)[0] || []).slice(-1)[0], [
		trace,
	]);

	return traceError ? (
		<Labeled label="Error" id="program-error">
			<output id="program-error" className={css.errorOutput}>
				error: {traceError}
			</output>
		</Labeled>
	) : (
		<React.Fragment>
			<Labeled label="Final Output" id="final-output">
				<output id="final-output" className={css.finalOutput}>
					{final}
				</output>
			</Labeled>
			<Labeled label="Minified Program" id="program-minified">
				<output id="program-minified" className={css.minifiedProgram}>
					{minifiedProgram}
				</output>
			</Labeled>
		</React.Fragment>
	);
}
