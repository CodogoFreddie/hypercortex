import * as R from "ramda";

const getAllKeysForId = (type, db) => id =>
	new Promise((done, fail) => {
		db.list(
			`data/${type}/${id}`,
			{
				recursive: true,
			},
			(err, dat) => (err ? fail(err) : done(dat.map(R.prop("key")))),
		);
	});

const deleteKeys = db => keys =>
	Promise.all(
		keys.map(
			key =>
				new Promise((done, fail) => {
					db.del(key, err => (err ? fail(err) : done()));
				}),
		),
	).then(() => keys);

const createDeleteHandler = (type, db, id) => ({
	delete: () =>
		Promise.resolve(id)
			.then(getAllKeysForId(type, db))
			.then(deleteKeys(db))
});

export default createDeleteHandler;
