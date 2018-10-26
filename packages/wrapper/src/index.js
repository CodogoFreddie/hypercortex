import * as R from "ramda";
import discovery from "discovery-swarm";
import swarmDefaults from "dat-swarm-defaults";
import openport from "openport";

const nodesToObj = path =>
	R.reduce(
		(acc, { key, value: { value, modifiedBy, modifiedAt } }) => (
			console.log({ acc, key, value }),
			{
				...acc,

				[key.replace(path, "")]: value,

				...(acc.modifiedAt < modifiedAt
					? {}
					: {
							modifiedAt,
							modifiedBy,
					  }),
			}
		),
		{},
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
				value: value => ({
					value,
					modifiedAt: new Date().toISOString(),
					modifiedBy: db.local.key.toString("hex"),
				}),
			}),
		),
	);

export const isAuthorised = db =>
	new Promise((done, fail) =>
		db.authorized(
			db.local.key,
			(err, auth) => (err ? fail(err) : done(auth)),
		),
	);

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

export const setObj = R.curry(async (db, type, id, obj) => {
	const isAuthed = await isAuthorised(db);
	if (!isAuthed) {
		console.log("not currently authed");
		return;
	}

	return new Promise((done, fail) => {
		const commands = objectToBatch(db, type, id)(obj);
		db.batch(commands, (err, nodes) => {
			if (err) {
				return fail(err);
			} else {
				done(nodes);
			}
		});
	});
});

export const createObj = R.curry((db, type, id) =>
	setObj(db, type, id, {
		createdAt: new Date().toISOString(),
		createdBy: db.local.key.toString("hex"),
	}),
);

//TODO find a way to make this into a propper itterator
export const getObjs = R.curry(async function*(db, type) {
	const objs = await new Promise((done, fail) => {
		db.list(
			`data/${type}/`,
			{
				recursive: false,
			},
			(err, data) => (err ? fail(err) : done(data)),
		);
	});

	for (const { key } of objs) {
		yield key.split("/")[2];
	}
});
