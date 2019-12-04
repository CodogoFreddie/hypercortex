import React from "react";
import { useRouter } from "next/router";
import { atob, btoa } from "isomorphic-base64";

import css from "./styles.css";

import EditorSetup from "../EditorSetup";

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
	const [exampleTask, setExampleTask] = React.useState({});
	const [stackStart, setStackStart] = React.useState("");
	const [program, setProgram] = React.useState(
		parseProgram(query.program || stringifyProgram([])),
	);
	const [trace, setTrace] = React.useState([]);

	React.useEffect(() => {
		try {
			const trace = stackMachineTracerRef.current(exampleTask, [
				stackStart,
				...program,
			]);

			const traceExcludingStart = trace.slice(
				stackStart.split(/\s+/gm).length - 1,
			);
			setTrace(traceExcludingStart);
		} catch (e) {
			console.error(e);
		}
	}, [exampleTask, stackStart, program]);

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
			<EditorSetup
				stackStart={stackStart}
				stackStartOnChange={setStackStart}
				exampleTask={exampleTask}
				exampleTaskOnChange={setExampleTask}
			/>
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
