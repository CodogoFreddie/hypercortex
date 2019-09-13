import React from "react";
import { ThemeProvider } from "emotion-theming";
import styled from "@emotion/styled";
import {
	calm,
	corePallete,
	buildScheme,
} from "@freddieridell/little-bonsai-styles";
import * as R from "ramda";

import useHyperTask from "./useHyperTask";

import TaskList from "./components/TaskList";
import Modal from "./components/Modal";
import Config, { useConfig } from "./components/Config";
import Topbar from "./components/Topbar";

//{"Create":[{"SetProp":{"Description":"test"}}]}

const theme = buildScheme(corePallete);

const Shell = styled.div(
	calm({
		display: "flex",
		flexDirection: "column",
		width: "100%",
		minHeight: "100%",
		backgroundColor: R.path(["theme", "color", "symantic", "background"]),
		color: R.path(["theme", "color", "symantic", "text"]),
	}),
);

const App = () => {
	const { config, onChangeConfig, configHasBeenSet } = useConfig();

	const { clientState, query, tasks, setQuery, runMutation } = useHyperTask(
		config,
	);

	return (
		<ThemeProvider theme={theme}>
			<Shell>
				<Topbar query={query} setQuery={setQuery} />
				{clientState !== "LOADING" && (
					<TaskList {...{ tasks, setQuery, runMutation, query }} />
				)}

				{!configHasBeenSet && (
					<Modal>
						<Config value={config} onChange={onChangeConfig} />
					</Modal>
				)}
			</Shell>
		</ThemeProvider>
	);
};

export default App;
