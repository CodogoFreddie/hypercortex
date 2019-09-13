import * as R from "ramda";

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
	return fetch(url)
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
							(remoteTask &&
								localTask &&
								localTask.updated_at > remoteTask.updated_at)
						) {
							return fetch(url, {
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

						done();
					}),
			);
		})
		.then(x => Promise.all(x));
}
