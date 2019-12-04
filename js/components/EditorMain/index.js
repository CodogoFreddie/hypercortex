import React from "react";
import cs from "classname";

import css from "./styles.css";

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
			<textarea
				className={cs(css.box, css.program)}
				value={stringifiedProgram}
				onChange={e => programOnChange(e.target.value.split("\n"))}
			/>
			<output className={cs(css.box, css.stackTrace)}>
				{formattedTrace}
			</output>
		</section>
	);
}
