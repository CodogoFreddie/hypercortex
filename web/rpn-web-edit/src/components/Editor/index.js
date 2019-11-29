import React from "react";
import { useRouter } from "next/router";
import { atob, btoa } from "isomorphic-base64";

import css from "./styles.css";

function stringifyProgram(p) {
	return btoa(JSON.stringify(p));
}

function parseProgram(x) {
	return JSON.parse(atob(x));
}

export default function Editor({ query }) {
	const router = useRouter();
	const [stackStart, setStackStart] = React.useState("");

	const [program, setProgram] = React.useState(
		parseProgram(query.program || stringifyProgram([])),
	);
	React.useEffect(() => {
		router.push({
			...router,
			query: {
				program: stringifyProgram(program),
			},
		});
	}, [program]);

	const minifiedProgram = React.useMemo(
		() => program.join(" ").replace(/\s+/gm, " "),
		[program],
	);

	return (
		<main className={css.editorContainer}>
			<section className={css.setupContainer}>
				<input
					value={stackStart}
					onChange={e => setStackStart(e.target.value)}
				/>
			</section>
			<section className={css.workspaceContainer}>
				<textarea
					value={program.join("\n")}
					onChange={e => setProgram(e.target.value.split("\n"))}
				/>
			</section>
			<output className={css.output}> {minifiedProgram} </output>
		</main>
	);
}
