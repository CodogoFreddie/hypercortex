import * as R from "ramda";
//const syncDbWithRemote = React.useCallback(() => {
//.then(tasks =>
//tasks.map(
//remoteTask =>
//new Promise((done, fail) => {
//const transaction = dbRef.current.transaction(
//["tasks"],
//"readwrite",
//);

//transaction.onerror = event => {
//console.error(
//"transaction.onerror",
//remoteTask,
//event,
//);
//};

//const objectStore = transaction.objectStore(
//"tasks",
//);

//const getRequest = objectStore.get(remoteTask.id);

//getRequest.onsuccess = function(event) {
//const localTask = getRequest.result;

//if (!localTask) {
//if there is no local task
//const addRequest = objectStore.add(
//remoteTask,
//);

//addRequest.onsuccess = done;
//}

//if (
//localTask.updated_at > remoteTask.updated_at
//) {
//}

//if (
//localTask.updated_at < remoteTask.updated_at
//) {
//const putRequest = objectStore.put(
//remoteTask,
//);

//putRequest.onsuccess = done;
//}
//};
//}),
//),
//)
//.then(ps => Promise.all(ps));
//}, [dbRef]);

function getObjectStore(db, meta) {
	const transaction = db.transaction(["tasks"], "readwrite");

	transaction.onerror = event => {
		console.error("transaction.onerror", meta, event);
	};

	const objectStore = transaction.objectStore("tasks");

	return objectStore;
}

function getAllLocalTasks(db) {
	const objectStore = getObjectStore(db, "getAllLocalTasks");

	return new Promise((done, fail) => {
		const requestGetAll = objectStore.getAll();

		requestGetAll.onsuccess = function(event) {
			done(R.indexBy(R.prop("id"), requestGetAll.result));
		};
	});
}

function getAllRemoteTasks(url) {
	return fetch("http://localhost:4523/")
		.then(x => x.json())
		.then(R.indexBy(R.prop("id")));
}

export default function syncDbWithServer(db, url) {
	return Promise.all([getAllLocalTasks(db), getAllRemoteTasks(url)])
		.then(([localTasksObj, remoteTasksObj]) => {
			const allIds = R.union(
				R.keys(localTasksObj),
				R.keys(remoteTasksObj),
			);

			const pairedTasks = allIds.reduce(
				(acc, id) => [...acc, [localTasksObj[id], remoteTasksObj[id]]],
				[],
			);

			return pairedTasks.map(
				([localTask, remoteTask]) =>
					new Promise((done, fail) => {
						if (
							!localTask ||
							localTask.updated_at < remoteTask.updated_at
						) {
							const putRequest = getObjectStore(db).put(
								remoteTask,
							);

							putRequest.onsuccess = done;
							putRequest.onerror = fail;
						}

						if (
							!remoteTask ||
							localTask.updated_at > remoteTask.updated_at
						) {
							fetch("http://localhost:4523/", {
								method: "POST",
								body: JSON.stringify(task),
								headers: {
									Accept: "application/json",
									"Content-Type": "application/json",
								},
							})
								.then(done)
								.catch(fail);
						}
					}),
			);
		})
		.then(x => Promise.all(x));
}
