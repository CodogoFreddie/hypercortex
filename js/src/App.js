import React from "react";

import { run as runHypertask } from "../../rust/hypertask_npm_package/Cargo.toml";
import HypertaskContext from "./HypertaskContext";
import TaskList from "./TaskList";

function useHyperTask() {
	React.useEffect(() => {
		if (navigator.storage && navigator.storage.persist) {
			navigator.storage.persisted().then(persisted => {
				if (!persisted) {
					navigator.storage.persist();
				}
			});
		}
	}, []);

	const [taskCmdResponse, setTaskCmdResponse] = React.useState([]);
	const [loading, setLoading] = React.useState(true);
	const dbRef = React.useRef(null);

	const openRequest = window.indexedDB.open("hypertask", 1);

	openRequest.onerror = function(event) {
		console.log("Why didn't you allow my web app to use IndexedDB?!");
	};
	openRequest.onsuccess = function(event) {
		const db = event.target.result;

		db.onerror = function(event) {
			console.error("Database error: " + event.target.errorCode);
		};

		dbRef.current = db;
		setLoading(false);
	};
	openRequest.onupgradeneeded = function(event) {
		const db = event.target.result;
		const objectStore = db.createObjectStore("tasks", { keyPath: "id" });

		objectStore.createIndex("id", "id", { unique: true });

		objectStore.transaction.oncomplete = function(event) {
			dbRef.current = db;
			setLoading(false);
		};
	};

	const runCommand = cmd => {
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
	};

	return {
		loading,
		tasks: taskCmdResponse,
		runCommand,
	};
}

const App = () => {
	const { loading, tasks, runCommand } = useHyperTask();

	React.useEffect(
		() =>
			runCommand({
				Read: [],
			}),
		[],
	);

	return (
		<HypertaskContext.Provider value={{ tasks, runCommand }}>
			{!loading && <TaskList />}
		</HypertaskContext.Provider>
	);
};

export default App;
