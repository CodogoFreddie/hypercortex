import * as R from "ramda";
import hash from "object-hash";

import resolveNodeConflict from "./resolveNodeConflict";

const createCollectionHandlers = (type, collections, db, id) => {
	return collections.map(prop => ({
		[`${prop}Get`]: () => {
			return new Promise((done, fail) => {
				db.list(`data/${type}/${id}/${prop}/`, (err, dat) => {
					err
						? fail(err)
						: done(
								R.pipe(
									R.map(resolveNodeConflict),
									R.map(R.path(["value", "value"])),
								)(dat),
						  );
				});
			});
		},

		[`${prop}Add`]: value => {
			const key = hash(value);
			return new Promise((done, fail) => {
				db.put(
					`data/${type}/${id}/${prop}/${key}`,
					{
						modifiedAt: new Date().toISOString(),
						modifiedBy: db.local.key.toString("hex"),
						value,
					},
					(err, dat) => (err ? fail(err) : done(dat)),
				);
			});
		},

		[`${prop}Remove`]: value => {
			const key = hash(value);
			return new Promise((done, fail) => {
				db.del(`data/${type}/${id}/${prop}/${key}`, (err, dat) =>
					err ? fail(err) : done(dat),
				);
			});
		},
	}));
};

export default createCollectionHandlers;
