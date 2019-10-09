import * as React from "react";

import { GIT_PASSWORD } from "../../.env";

import useHyperTaskEngine, { Task } from "../hooks/useHyperTaskEngine";

const emptyTaskArray: Task[] = [];
const emptyTaskSet = new Set(emptyTaskArray);

const Home = () => {
	const [isLoading, setIsLoading] = React.useState(true);

	const run = useHyperTaskEngine({
		remote: "https://github.com/FreddieRidell/cortex.git",
		kind: "git",
		username: "FreddieRidell",
		token: GIT_PASSWORD,
		scoringFunction: "due :",
	});

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
