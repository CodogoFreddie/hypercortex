import React from "react";

import { run as runHypertask } from "../../rust/hypertask_npm_package/Cargo.toml";

import HypertaskContext, { useHyperTask } from "./HypertaskContext";
import TaskList from "./TaskList";

const App = () => {
	const { loading, tasks, runCommand } = useHyperTask();

	React.useEffect(
		() =>
			runCommand({
				Read: [],
			}),
		[],
	);

	return (
		<HypertaskContext.Provider value={{ tasks, runCommand }}>
			{!loading && <TaskList />}
		</HypertaskContext.Provider>
	);
};

export default App;
