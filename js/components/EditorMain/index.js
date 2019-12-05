import React from "react";
import cs from "classname";

import Labeled from "../Labeled";

import css from "./styles.scss";

export default function EditorMain({ program, programOnChange, trace }) {
	const stringifiedProgram = React.useMemo(() => program.join("\n"), [
		program,
	]);

	const formattedTrace = React.useMemo(() => {
		const commandsPerLine = program.map(
			line => line.split(/\s+/gm).filter(({ length }) => length).length,
		);

		const traceIndexes = new Set(
			commandsPerLine.reduce(
				({ arr, count }, val) => ({
					arr: [...arr, count + val],
					count: count + val,
				}),
				{ arr: [], count: 0 },
			).arr,
		);

		return trace
			.filter((_, i) => traceIndexes.has(i))
			.map(x => x.join("  "))
			.join("\n");
	}, [trace]);

	return (
		<section className={css.container}>
			<Labeled
				id="program"
				label="Program"
				classNameSection={css.programContainer}
			>
				<textarea
					id="program"
					className={css.program}
					value={stringifiedProgram}
					onChange={e => programOnChange(e.target.value.split("\n"))}
				/>
			</Labeled>

			<Labeled
				id="stack-trace"
				label="Stack Trace"
				classNameSection={css.stackTraceContainer}
			>
				<output id="stack-trace" className={css.stackTrace}>
					{formattedTrace}
				</output>
			</Labeled>
		</section>
	);
}
