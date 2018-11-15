import * as R from "ramda";
import hash from "object-hash";

import easyTypeId from "@hypercortex/easy-type-id";

const resolveNodeConflict = R.reduce(
	(l, r) => (l.value.modifiedAt > r.value.modifiedAt ? l : r),
	{ value: { modifiedAt: "" } },
);

const createScalarHandlers = (type, scalars, db, id) =>
	scalars.map(prop => ({
		[`${prop}Get`]: () =>
			new Promise((done, fail) =>
				db.get(`/data/${type}/${id}/${prop}`, (err, dat) =>
					err ? fail(err) : done(dat),
				),
			)
				.then(resolveNodeConflict)
				.then(R.path(["value", "value"])),

		[`${prop}Set`]: value => {
			return new Promise((done, fail) =>
				db.put(
					`/data/${type}/${id}/${prop}`,
					{
						modifiedAt: new Date().toISOString(),
						modifiedBy: db.local.key.toString("hex"),
						value,
					},
					(err, dat) => (err ? fail(err) : done(dat)),
				),
			);
		},
	}));

const createCollectionHandlers = (type, collections, db, id) => {
	return collections.map(prop => ({
		[`${prop}Get`]: () => {
			return new Promise((done, fail) => {
				db.list(`/data/${type}/${id}/${prop}/`, (err, dat) => {
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
					`/data/${type}/${id}/${prop}/${key}`,
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
				db.del(`/data/${type}/${id}/${prop}/${key}`, (err, dat) =>
					err ? fail(err) : done(dat),
				);
			});
		},
	}));
};

const createObjecTypeWrapper = R.curry(
	(
		type,
		{
			scalars = [],
			collections = [],
			relations: { one = [], many = [] } = {},
		},
		db,
		id,
	) =>
		Object.assign(
			{
				toObj: (depth = 0) => {},
			},
			...createScalarHandlers(type, scalars, db, id),
			...createCollectionHandlers(type, collections, db, id),
		),
);

export default createObjecTypeWrapper;
