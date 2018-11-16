import * as R from "ramda";
import hash from "object-hash";

import resolveNodeConflict from "./resolveNodeConflict";

const createCollectionHandlers = (type, collections, db, id) =>
	Object.assign(
		{},
		...collections.map(collection => {
			const { name, sortBy, shouldSort } =
				typeof collection === "string"
					? {
							name: collection,
							sortBy: R.identity,
							shouldSort: false,
					  }
					: {
							...collection,
							shouldSort: true,
					  };

			return {
				[`${name}Get`]: () => {
					return new Promise((done, fail) => {
						db.list(`data/${type}/${id}/${name}/`, (err, dat) => {
							err
								? fail(err)
								: done(
										R.pipe(
											R.map(resolveNodeConflict),
											R.map(R.path(["value", "value"])),
											shouldSort
												? R.sortBy(sortBy)
												: R.identity,
										)(dat),
								  );
						});
					});
				},

				[`${name}Add`]: value => {
					const key = hash(value);
					return new Promise((done, fail) => {
						db.put(
							`data/${type}/${id}/${name}/${key}`,
							{
								modifiedAt: new Date().toISOString(),
								modifiedBy: db.local.key.toString("hex"),
								value,
							},
							(err, dat) => (err ? fail(err) : done(dat)),
						);
					});
				},

				[`${name}Remove`]: value => {
					const key = hash(value);
					return new Promise((done, fail) => {
						db.del(
							`data/${type}/${id}/${name}/${key}`,
							(err, dat) => (err ? fail(err) : done(dat)),
						);
					});
				},
			};
		}),
	);

export default createCollectionHandlers;
