export default function getDb() {
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
