import * as R from "ramda";

const nodesToObj = path =>
	R.pipe(
		R.map(
			R.pipe(
				R.nth(0),
				({ key, value }) => [key.replace(path, ""), value],
			),
		),
		R.fromPairs,
	);

const objectToBatch = (db, type, id) =>
	R.pipe(
		R.toPairs,
		R.map(([key, value]) => ({
			type: "put",
			key,
			value,
		})),
		R.map(
			R.evolve({
				key: key => `data/${type}/${id}/${key}`,
			}),
		),

		R.append({
			type: "put",
			key: `data/${type}/${id}/modifiedAt`,
			value: new Date().toISOString(),
		}),

		R.append({
			type: "put",
			key: `data/${type}/${id}/modifiedBy`,
			value: db.local.key.toString("hex"),
		}),
	);

export const createReducer = conflictResolvers => (a, b) => {
	console.log(a, b);
	return a;
};

export const readyGate = db =>
	new Promise((done, fail) => {
		db.on("ready", done());
	});

export const getObj = R.curry(
	(db, type, id) =>
		new Promise((done, fail) => {
			const path = `data/${type}/${id}/`;
			db.list(path, (err, nodes) => {
				if (err) {
					fail(err);
				} else {
					done(nodesToObj(path)(nodes));
				}
			});
		}),
);

export const setObj = R.curry(
	(db, type, id, obj) =>
		new Promise((done, fail) => {
			const commands = objectToBatch(db, type, id)(obj);
			db.batch(commands, (err, nodes) => {
				if (err) {
					return fail(err);
				} else {
					done(nodes);
				}
			});
		}),
);

export const createObj = R.curry((db, type, id) =>
	setObj(db, type, id, {
		createdAt: new Date().toISOString(),
		createdBy: db.local.key.toString("hex"),
	}),
);

const readNextStreamChunk = stream =>
	new Promise((done, fail) => {
		stream.on("data", data => {
			done(data);
		});
	});

//TODO find a way to make this into a propper itterator
export const getObjs = R.curry(async function*(db, type) {
	const objs = await new Promise((done, fail) =>
		db.list(
			`data/${type}/`,
			{
				recursive: false,
			},
			(err, data) => (err ? fail(err) : done(data)),
		),
	);

	for (const [{ key }] of objs) {
		yield key.split("/")[2];
	}
});
