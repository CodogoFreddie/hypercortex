import initHypertask, { run as runHypertask } from "hypertask_npm_package";

import React from "react";

const HypertaskContext = React.createContext({
	loading: true,
	tasks: {},
	runCommand: () => {},
});

function useHyperTask() {
	const [taskCmdResponse, setTaskCmdResponse] = useState([]);

	const runCommand = cmd => {
		let outputTasks = runHypertask(cmd, console.log, new Set([]).values());
		setTaskCmdResponse(outputTasks);
	};

	return { tasks: taskCmdResponse, runCommand };
}

const App = () => {
	const { tasks, runCommand } = useHyperTask();

	useEffect(
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

	console.log({ tasks });

	return (
		<HypertaskContext.Provider>
			<div />
		</HypertaskContext.Provider>
	);
};
