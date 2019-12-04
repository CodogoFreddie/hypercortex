import React from "react";

export default function EditorMain({ program, programOnChange, trace }) {
	const stringifiedProgram = React.useMemo(() => program.join("\n"), [
		program,
	]);

	return (
		<section>
			<textarea
				value={stringifiedProgram}
				onChange={e => programOnChange(e.target.value.split("\n"))}
			/>
		</section>
	);
}
