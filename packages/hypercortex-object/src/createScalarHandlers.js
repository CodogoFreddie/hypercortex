import * as R from "ramda";

const createScalarHandlers = (type, scalars, db, id) =>
	Object.assign(
		{},
		...scalars.map(prop => ({
			[`${prop}Get`]: () =>
				new Promise((done, fail) =>
					db.get(`data/${type}/${id}/${prop}`, (err, dat) =>
						err ? fail(err) : done(dat),
					),
				)
					.then(R.path(["value", "value"])),

			[`${prop}Set`]: value => {
				return new Promise((done, fail) =>
					db.put(
						`data/${type}/${id}/${prop}`,
						{
							modifiedAt: new Date().toISOString(),
							modifiedBy: db.local.key.toString("hex"),
							value,
						},
						(err, dat) => (err ? fail(err) : done(dat)),
					),
				);
			},

			[`${prop}Delete`]: value => {
				return new Promise((done, fail) =>
					db.put(
						`data/${type}/${id}/${prop}`,
						null,
						(err, dat) => (err ? fail(err) : done(dat)),
					),
				);
			},
		})),
	);

export default createScalarHandlers;
