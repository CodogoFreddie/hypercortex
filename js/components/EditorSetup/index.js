import React from "react";
import cs from "classname";

import css from "./styles.css";

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
		<section className={css.container}>
			<label for="stack-start">Initial Stack State</label>
			<input
				id="stack-start"
				className={cs(css.input, css.stackStart)}
				value={stackStart}
				onChange={e => stackStartOnChange(e.target.value)}
			/>
			<label for="example-task">Input Test Task</label>
			<textarea
				rows={14}
				cols={80}
				id="example-task"
				data-valid={isValid}
				className={cs(css.input, css.exampleTask)}
				value={taskStringified}
				onChange={e => taskStringifiedSet(e.target.value)}
			/>
		</section>
	);
}
