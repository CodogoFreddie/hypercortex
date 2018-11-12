const createObjecTypeWrapper = (type, { scalars, sets }) => db => id => ({
	...scalars.reduce(
		(acc, prop) => ({
			...acc,

			get [prop]() {
				return new Promise((done, fail) =>
					db.get(`/data/${type}/${id}/${prop}`, (err, dat) =>
						err ? fail(err) : done(dat),
					),
				);
			},

			set [prop](value) {
				return new Promise((done, fail) =>
					db.get(`/data/${type}/${id}/${prop}`, (err, dat) =>
						err ? fail(err) : done(dat),
					),
				);
			},
		}),
		{},
	),
});
