import React from "react";

import getIDB from "./getIDB";
import syncDbWithServer from "./syncDbWithServer";
import useDataPersistence from "./useDataPersistence";

const HypertaskContext = React.createContext({
	tasks: {},
	runCommand: () => {},
});

export default function useHyperTask() {
	useDataPersistence();

	const [clientState, setClientState] = React.useState("LOADING");
	const [tasks, setTasks] = React.useState([]);
	const [query, setQuery] = React.useState([]);

	const dbRef = React.useRef(null);

	const runCommand = React.useCallback(
		cmd => {
			if (!dbRef.current) {
				return;
			}

			dbRef.current
				.transaction("tasks")
				.objectStore("tasks")
				.getAll().onsuccess = event => {
				import("@freddieridell/hypertask_npm_package").then(
					({ run: runHypertask }) => {
						const outputTasks = runHypertask(
							cmd,
							task => {
								const transaction = dbRef.current.transaction(
									["tasks"],
									"readwrite",
								);

								transaction.onerror = event => {
									console.error(
										"transaction.onerror",
										task,
										event,
									);
								};

								const objectStore = transaction.objectStore(
									"tasks",
								);
								const request = objectStore.add(task);
							},
							event.target.result.values(),
						);

						setTasks(outputTasks);
					},
				);
			};
		},
		[dbRef],
	);

	React.useEffect(() => {
		runCommand({
			Read: query,
		});
	}, [query]);

	const syncDbWithRemote = React.useCallback(() => {
		setClientState("SYNCING");

		return syncDbWithServer(dbRef.current, localStorage.apiURL).then(() => {
			setClientState("SYNCED");
		});
	}, [dbRef]);

	React.useEffect(
		//load the db
		() => {
			getIDB()
				.then(db => {
					dbRef.current = db;
				})
				.then(() => {
					runCommand({
						Read: [],
					});
				})
				.then(() => syncDbWithRemote())
				.then(() => {
					runCommand({
						Read: [],
					});
				})
				.catch(console.error);
		},
		[syncDbWithRemote],
	);

	return {
		clientState,
		query,
		setQuery,
		tasks,
	};
}
