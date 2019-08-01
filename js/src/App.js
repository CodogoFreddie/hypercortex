import React from "react";

import { run as runHypertask } from "../../rust/hypertask_npm_package/Cargo.toml";
import HypertaskContext from "./HypertaskContext";
import TaskList from "./TaskList";

function useHyperTask() {
	const [taskCmdResponse, setTaskCmdResponse] = React.useState([]);

	const runCommand = cmd => {
		let outputTasks = runHypertask(cmd, console.log, new Set([]).values());
		setTaskCmdResponse(outputTasks);
	};

	return {
		tasks: taskCmdResponse,
		runCommand,
	};
}

const App = () => {
	const { tasks, runCommand } = useHyperTask();

	React.useEffect(
		() =>
			runCommand({
				Create: [
					{
						SetProp: {
							Description: "Hello World",
						},
					},
				],
			}),
		[],
	);

	console.log({
		tasks,
	});

	return (
		<HypertaskContext.Provider value={{ tasks, runCommand }}>
			<TaskList />
		</HypertaskContext.Provider>
	);
};

export default App;
