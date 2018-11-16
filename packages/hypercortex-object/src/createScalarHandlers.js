import * as R from "ramda";

import resolveNodeConflict from "./resolveNodeConflict";

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
					.then(resolveNodeConflict)
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
		})),
	);

export default createScalarHandlers;
