import React from "react";
import { ThemeProvider } from "emotion-theming";
import { corePallete, buildScheme } from "@freddieridell/little-bonsai-styles";

import useHyperTask from "./useHyperTask";

import QueryRenderer from "./components/QueryRenderer";
import TaskList from "./components/TaskList";
import Config, { configHasBeenSet } from "./components/Config";

//{"Create":[{"SetProp":{"Description":"test"}}]}

const theme = buildScheme(corePallete);

const App = () => {
	const { clientState, query, tasks, setQuery, runMutation } = useHyperTask();

	if (!configHasBeenSet()) {
		return <Config />;
	}

	return (
		<ThemeProvider theme={theme}>
			<QueryRenderer query={query} setQuery={setQuery} />
			{clientState !== "LOADING" && (
				<TaskList {...{ tasks, setQuery, runMutation, query }} />
			)}
		</ThemeProvider>
	);
};

export default App;
