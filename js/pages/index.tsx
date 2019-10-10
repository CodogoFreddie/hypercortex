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

	console.log({ config, Router });
	if (!config && process.browser) {
		Router.push("/config");

		return null;
	}

	const run = useHyperTaskEngine(config);

	React.useEffect(() => {
		if (!run) {
			return;
		}

		try {
			run(
				{ Create: [{ SetProp: { Description: "test" } }] },
				task => {},
				emptyTaskSet.values()
			);
		} catch (e) {
			console.error(e);
		}
	}, [run]);

	return <div>home</div>;
};

export default Home;
