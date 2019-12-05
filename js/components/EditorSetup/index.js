import React from "react";
import cs from "classname";

import Labeled from "../Labeled";

import css from "./styles.scss";

export default function EditorSetup({
	stackStart,
	stackStartOnChange,
	exampleTask,
	exampleTaskOnChange,
}) {
	const [taskStringified, taskStringifiedSet] = React.useState(
		JSON.stringify(exampleTask, null, 2),
	);
	const [isValid, isValidSet] = React.useState(true);

	React.useEffect(() => {
		taskStringifiedSet(JSON.stringify(exampleTask, null, 2));
	}, [exampleTask]);

	React.useEffect(() => {
		try {
			const parsed = JSON.parse(taskStringified);
			exampleTaskOnChange(parsed);
			isValidSet(true);
		} catch (e) {
			isValidSet(false);
		}
	}, [taskStringified]);

	return (
		<section>
			<Labeled label="Initial Stack State" id="stack-start">
				<input
					id="stack-start"
					className={css.stackStart}
					value={stackStart}
					onChange={e => stackStartOnChange(e.target.value)}
				/>
			</Labeled>
			<Labeled label="Input Test Task" htmlFor="example-task">
				<textarea
					rows={14}
					cols={80}
					id="example-task"
					data-valid={isValid}
					className={css.exampleTask}
					value={taskStringified}
					onChange={e => taskStringifiedSet(e.target.value)}
				/>
			</Labeled>
		</section>
	);
}
