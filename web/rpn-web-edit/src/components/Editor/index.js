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

function useRPNTracer() {
	const stackMachineTracer = React.useRef();

	React.useEffect(() => {
		import("../../../../../rust/hypertask_client_js/pkg").then(
			({ get_machine_stack_trace }) => {
				stackMachineTracer.current = get_machine_stack_trace;
			},
		);
	}, []);

	return stackMachineTracer;
}

export default function Editor({ query }) {
	const stackMachineTracerRef = useRPNTracer();

	const router = useRouter();
	const [testTaskString, setTestTaskString] = React.useState("");
	const [stackStart, setStackStart] = React.useState("");
	const [program, setProgram] = React.useState(
		parseProgram(query.program || stringifyProgram([])),
	);
	const [trace, setTrace] = React.useState([]);

	React.useEffect(() => {
		try {
			const trace = stackMachineTracerRef.current(
				JSON.parse(testTaskString),
				[stackStart, ...program],
			);

			setTrace(trace);
		} catch (e) {
			console.error(e);
		}
	}, [testTaskString, stackStart, program]);

	React.useEffect(() => console.log(trace), [trace]);

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
				<textarea
					value={testTaskString}
					onChange={e => setTestTaskString(e.target.value)}
				/>
			</section>
			<section className={css.workspaceContainer}>
				<textarea
					value={program.join("\n")}
					onChange={e => setProgram(e.target.value.split("\n"))}
				/>
			</section>
			<output className={css.output}> {minifiedProgram} </output>
			<output style={{ whiteSpace: "pre" }}>
				{trace.map(x => x.join("\t")).join("\n")}
			</output>
		</main>
	);
}
