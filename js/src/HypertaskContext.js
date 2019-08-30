import React from "react";
import { run as runHypertask } from "../../rust/hypertask_npm_package/src/lib.rs";

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

	const runCommand = cmd => {
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

					fetch("http://localhost:4523/", {
						method: "POST",
						body: JSON.stringify(task),
						headers: {
							Accept: "application/json",
							"Content-Type": "application/json",
						},
					});
				},
				event.target.result.values(),
			);

			setTaskCmdResponse(outputTasks);
		};
	};

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
				.then(() => fetch("http://localhost:4523/"))
				.then(x => x.json())
				.then(tasks => {
					tasks.forEach(task => {
						const transaction = dbRef.current.transaction(
							["tasks"],
							"readwrite",
						);

						transaction.onerror = event => {
							console.error("transaction.onerror", task, event);
						};

						const objectStore = transaction.objectStore("tasks");

						const request = objectStore.put(task);
					});
				})
				.then(() => {
					runCommand({
						Read: [],
					});
				})
				.catch(console.error);
		},
		[],
	);

	return {
		loading: !dbRef.current,
		tasks: taskCmdResponse,
		runCommand,
	};
}
