import React from "react";
import { useRouter } from "next/router";
import { atob, btoa } from "isomorphic-base64";

import EditorSetup from "../EditorSetup";
import EditorMain from "../EditorMain";
import EditorOutput from "../EditorOutput";

import { useLocalStorageState } from "../../hooks";

import css from "./styles.scss";

function stringifyProgram(p) {
	return btoa(JSON.stringify(p));
}

function parseProgram(x) {
	return JSON.parse(atob(x));
}

function useRPNTracer() {
	const stackMachineTracer = React.useRef();
	const [loaded, loadedSet] = React.useState(false);

	React.useEffect(() => {
		import("@freddieridell/hypertask_client_js").then(
			({ get_machine_stack_trace }) => {
				stackMachineTracer.current = get_machine_stack_trace;
				loadedSet(true);
			},
		);
	}, []);

	return [stackMachineTracer, loaded];
}

function useProgram(query) {
	const router = useRouter();
	const [program, programSet] = React.useState(
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

	return [program, programSet];
}

function useExampleTask() {}

export default function Editor({ query }) {
	const [stackMachineTracerRef, stackMachineLoaded] = useRPNTracer();

	const [program, programSet] = useProgram(query);
	const [exampleTask, setExampleTask] = useLocalStorageState(
		"exampleTask",
		{},
	);
	const [stackStart, setStackStart] = useLocalStorageState("stackStart", "");
	const [trace, setTrace] = React.useState([]);
	const [traceError, traceErrorSet] = React.useState(null);

	React.useEffect(() => {
		if (!stackMachineLoaded) {
			return;
		}

		try {
			const trace = stackMachineTracerRef.current(exampleTask, [
				stackStart,
				...program,
			]);

			const traceExcludingStart = trace.slice(
				stackStart.split(/\s+/gm).length - 1,
			);
			setTrace(traceExcludingStart);
			traceErrorSet(null);
		} catch (e) {
			traceErrorSet(e.toString());
		}
	}, [exampleTask, stackStart, program, stackMachineLoaded]);

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

			<EditorMain
				program={program}
				programOnChange={programSet}
				trace={trace}
			/>

			<EditorOutput
				final={trace.slice(-1)}
				minifiedProgram={minifiedProgram}
				traceError={traceError}
			/>
		</main>
	);
}
