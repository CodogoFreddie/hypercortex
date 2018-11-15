import * as R from "ramda";
import hash from "object-hash";

import easyTypeId from "@hypercortex/easy-type-id";

const resolveNodeConflict = R.reduce((l, r) =>
	l.value.modifiedAt > r.value.modifiedAt ? l : r,
);

const createScalarHandlers = (type, scalars, db, id) =>
	scalars.map(prop => ({
		get [prop]() {
			return new Promise((done, fail) =>
				db.get(`/data/${type}/${id}/${prop}`, (err, dat) =>
					err ? fail(err) : done(dat),
				),
			).then(resolveNodeConflict.value.value);
		},

		set [prop](value) {
			return new Promise((done, fail) =>
				db.set(
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
		get [prop]() {
			return new Promise((done, fail) => {
				db.list(`/data/${type}/${id}/${prop}/`, (err, dat) => {
					err
						? fail(err)
						: done(
								dat
									.map(resolveNodeConflict)
									.map(R.path(["value", "value"])),
						  );
				});
			});
		},

		set [prop](value) {
			throw `error setting ${prop} on type ${type}: can not directly assign to collections, please use ${prop}Add(x), and ${prop}Remove(x)`;
		},

		[`${prop}Add`]: input => {
			const key = hash(input);
			return new Promise((done, fail) => {
				db.set(
					`/data/${type}/${id}/${prop}/${key}`,
					{
						modifiedAt: new Date().toISOString(),
						modifiedBy: db.local.key.toString("hex"),
						input,
					},
					(err, dat) => (err ? fail(err) : done(dat)),
				);
			});
		},

		[`${prop}Remove`]: input => {
			const key = hash(input);
			return new Promise((done, fail) => {
				db.del(`/data/${type}/${id}/${prop}/${key}`, (err, dat) =>
					err ? fail(err) : done(dat),
				);
			});
		},
	}));
};

const createObjecTypeWrapper = R.curry(
	(type, { scalars, collections, relations: { one, many } }, db, id) =>
		Object.assign(
			{
				toObj: (depth = 0) => {},
			},
			...createScalarHandlers(type, scalars, db, id),
			...createCollectionHandlers(type, collections, db, id),
		),
);

export default createObjecTypeWrapper;
