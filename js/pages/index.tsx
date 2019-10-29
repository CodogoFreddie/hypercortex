import * as React from "react";
import Router from "next/router";

import { GIT_PASSWORD } from "../../.env";

import useHyperTaskEngine, { Task } from "../hooks/useHyperTaskEngine";
import useConfig from "../hooks/useConfig";

const emptyTaskArray: Task[] = [];
const emptyTaskSet = new Set(emptyTaskArray);

const Home = () => {
	const [isLoading, setIsLoading] = React.useState(true);

	const [config, setConfig, configIsComplete] = useConfig();

	if (!configIsComplete && process.browser) {
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
			{allTasks.map(({ score, task: { id, description } }) => (
				<div key={id}>
					{[score.toPrecision(3), description].join(" ")}
				</div>
			))}
		</div>
	);
};

export default Home;
