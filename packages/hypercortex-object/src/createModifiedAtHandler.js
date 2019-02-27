import * as R from "ramda";

const createModifiedAtHandler = (type, db, id) => ({
	modifiedAtGet: () =>
		new Promise((done, fail) => {
			db.list(
				`data/${type}/${id}`,
				{
					recursive: true,
				},
				(err, dat) =>
					err ? fail(err) : done(dat.map(R.prop("value")).map(R.prop("modifiedAt"))),
			);
		}).then(modifiedAtDates =>
			modifiedAtDates.reduce((l, r) => (l > r ? l : r)),
		),
});

export default createModifiedAtHandler;
