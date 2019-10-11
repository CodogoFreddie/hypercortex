import * as React from "react";
import Router from "next/router";

import { GIT_PASSWORD } from "../../.env";

import useHyperTaskEngine, { Task } from "../hooks/useHyperTaskEngine";
import useConfig from "../hooks/useConfig";

const emptyTaskArray: Task[] = [];
const emptyTaskSet = new Set(emptyTaskArray);

const Home = () => {
	const [isLoading, setIsLoading] = React.useState(true);

	const [config, setConfig] = useConfig();

	if (!config && process.browser) {
		Router.push("/config");

		return null;
	}

	const run = useHyperTaskEngine(config);

	const allTasks = React.useMemo(() => {
		if (!run) {
			return [];
		}

		return run({ Read: [] });
	}, [run]);

	return (
		<div>
			{allTasks.map(({ score, task: { description } }) => (
				<div>{[score, description].join(" ")}</div>
			))}
		</div>
	);
};

export default Home;
