import * as R from "ramda";

const resolveNodeConflict = R.pipe(
	R.reduce((l, r) => (l.modifiedAt > r.modifiedAt ? l : r)),
	R.prop("value"),
);

const createScalarHandlers = (type, scalars, db, id) =>
	scalars.reduce(
		(acc, prop) => ({
			...acc,

			get [prop]() {
				return new Promise((done, fail) =>
					db.get(
						`/data/${type}/${id}/${prop}`,
						(err, dat) => (err ? fail(err) : done(dat)),
					),
				).then(resolveNodeConflict);
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
		}),
		{},
	);

const createSetHandlers = (type, sets, db, id) => 
	sets.reduce(
		(acc, prop) => ({
			...acc,
		}),
		{}
	)

const createObjecTypeWrapper = R.curry(
	(type, { scalars, sets, relations: { one, many } }, db, id) => ({
		...createScalarHandlers(type, scalars, db, id),

		toObj: (depth = 0) => {

		}
	}),
);

export default createObjecTypeWrapper,
