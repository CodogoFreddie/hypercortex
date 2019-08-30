import React from "react";

import HypertaskContext, { useHyperTask } from "./HypertaskContext";
import TaskList from "./TaskList";

//{"Create":[{"SetProp":{"Description":"test"}}]}

const App = () => {
	const { loading, tasks, runCommand } = useHyperTask();

	return (
		<HypertaskContext.Provider value={{ tasks, runCommand }}>
			{!loading && <TaskList />}
		</HypertaskContext.Provider>
	);
};

export default App;
