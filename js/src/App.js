import React from "react";
import { ThemeProvider } from "emotion-theming";
import { corePallete, buildScheme } from "@freddieridell/little-bonsai-styles";

import HypertaskContext, { useHyperTask } from "./HypertaskContext";
import TaskList from "./TaskList";

//{"Create":[{"SetProp":{"Description":"test"}}]}

const theme = buildScheme(corePallete);

const App = () => {
	const { loading, tasks, runCommand } = useHyperTask();

	return (
		<ThemeProvider theme={theme}>
			<HypertaskContext.Provider value={{ tasks, runCommand }}>
				{!loading && <TaskList />}
			</HypertaskContext.Provider>
		</ThemeProvider>
	);
};

export default App;
