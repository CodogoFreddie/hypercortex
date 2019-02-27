import * as R from "ramda";

const createDeleteHandler = (type, db, id) => ({
	delete: () =>
		new Promise((done, fail) => {
			db.list(
				`data/${type}/${id}`,
				{
					recursive: true,
				},
				(err, dat) => (err ? fail(err) : done(dat.map(R.prop("key")))),
			);
		})
			.then(keys =>
				Promise.all(
					keys.map(
						key =>
							new Promise((done, fail) => {
								db.del(key, err => (err ? fail(err) : done()));
							}),
					),
				).then(() => keys),
			)
			.then(keys =>
				Promise.all(
					keys.map(
						key =>
							new Promise((done, fail) => {
								db.del(key, err => (err ? fail(err) : done()));
							}),
					),
				).then(() => keys),
			)
})

export default createDeleteHandler
