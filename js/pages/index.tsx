import * as React from "react";

import useHyperTaskEngine, { Task } from "../hooks/useHyperTaskEngine";

const emptyTaskArray: Task[] = [];
const emptyTaskSet = new Set(emptyTaskArray);

const Home = () => {
	const [isLoading, setIsLoading] = React.useState(true);

	const run = useHyperTaskEngine();

	console.log({ run });

	React.useEffect(() => {
		if (!run) {
			return;
		}

		try {
			run(
				{ Create: [{ SetProp: { Description: "test" } }] },
				task => console.log("update", { task }),
				emptyTaskSet.values()
			);
		} catch (e) {
			console.error(e);
		}
	}, [run]);

	return <div>home</div>;
};

export default Home;
