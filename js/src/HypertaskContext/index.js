import React from "react";
import { run as runHypertask } from "../../../rust/hypertask_npm_package/src/lib.rs";

import syncDbWithServer from "./syncDbWithServer";

const HypertaskContext = React.createContext({
	tasks: {},
	runCommand: () => {},
});

export default HypertaskContext;

function getDb() {
	return new Promise((done, fail) => {
		const openRequest = window.indexedDB.open("hypertask", 1);

		openRequest.onerror = function(event) {
			console.log("Why didn't you allow my web app to use IndexedDB?!");
			fail(event);
		};

		openRequest.onsuccess = function(event) {
			const db = event.target.result;

			db.onerror = function(event) {
				console.error("Database error: " + event.target.errorCode);
				fail();
			};

			done(db);
		};
		openRequest.onupgradeneeded = function(event) {
			const db = event.target.result;
			const objectStore = db.createObjectStore("tasks", {
				keyPath: "id",
			});

			objectStore.createIndex("id", "id", { unique: true });

			objectStore.transaction.oncomplete = function(event) {
				dbRef.current = db;
				setLoading(false);
			};
		};
	});
}

export function useHyperTask() {
	const [taskCmdResponse, setTaskCmdResponse] = React.useState([]);
	const dbRef = React.useRef(null);

	React.useEffect(
		//ask for data persistence
		() => {
			if (navigator.storage && navigator.storage.persist) {
				navigator.storage.persisted().then(persisted => {
					if (!persisted) {
						navigator.storage.persist();
					}
				});
			}
		},
		[],
	);

	const syncDbWithRemote = React.useCallback(() => {
		return syncDbWithServer(dbRef.current, "http://localhost:4523");
	}, [dbRef]);

	const runCommand = React.useCallback(
		cmd => {
			if (!dbRef.current) {
				return;
			}

			dbRef.current
				.transaction("tasks")
				.objectStore("tasks")
				.getAll().onsuccess = event => {
				const outputTasks = runHypertask(
					cmd,
					task => {
						const transaction = dbRef.current.transaction(
							["tasks"],
							"readwrite",
						);

						transaction.onerror = event => {
							console.error("transaction.onerror", task, event);
						};

						const objectStore = transaction.objectStore("tasks");
						const request = objectStore.add(task);
					},
					event.target.result.values(),
				);

				setTaskCmdResponse(outputTasks);
			};
		},
		[dbRef],
	);

	React.useEffect(
		//load the db
		() => {
			getDb()
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
		loading: !dbRef.current,
		tasks: taskCmdResponse,
		runCommand,
	};
}
