import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";
import discovery from "discovery-swarm";
import swarmDefaults from "dat-swarm-defaults";
import openport from "openport";

const nodesToObj = path =>
	R.pipe(
		R.map(R.pipe(({ key, value }) => [key.replace(path, ""), value])),
		R.fromPairs,
	);

const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);
const rename = util.promisify(fs.rename);

export const openDefaultDb = async name => {
	try {
		const publicKeyBuffer = await readFile(envpaths(name).config);
		const publicKey = publicKeyBuffer.toString();
		return hyperdb(
			envpaths(name).data + "/" + publicKey,
			Buffer.from(publicKey, "hex"),
			{
				valueEncoding: "json",
				reduce: (a, b) => {
					console.log("REDUCING!!!", { a, b });
					return a;
				},
			},
		);
	} catch (e) {
		const db = hyperdb(envpaths(name).data + "/temp", {
			valueEncoding: "json",
		});

		await readyGate(db);

		const publicKey = db.key.toString("hex");

		await rename(
			envpaths(name).data + "/temp",
			envpaths(name).data + "/" + publicKey,
		);
		await writeFile(envpaths(name).config, publicKey);

		return openDefaultDb(name);
	}
};

export const createReducer = conflictResolvers => (a, b) => {
	console.log(a, b);
	return a;
};

export const readyGate = db =>
	new Promise((done, fail) => {
		db.on("ready", done);
	});

export const getObj = R.curry(
	(db, type, id) =>
		new Promise((done, fail) => {
			const path = `data/${type}/${id}`;

			db.get(path, (err, { value }) => {
				console.log(value);
				err ? fail(err) : done(value);
			});
		}),
);

export const setObj = R.curry(
	(db, type, id, obj) =>
		new Promise((done, fail) => {
			db.put(
				`data/${type}/${id}`,
				{
					...obj,
					modifiedBy: db.local.key.toString("hex"),
					modifiedAt: new Date().toISOString(),
				},
				(err, nodes) => {
					if (err) {
						return fail(err);
					} else {
						done(nodes);
					}
				},
			);
		}),
);

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

export const justReplicate = R.curry((handlers, db) => {
	console.log("replicating", db.key.toString("hex"));

	openport.find({ startingPort: 15423 }, (err, port) => {
		console.log(`on port ${port}`);
		var swarm = discovery(swarmDefaults());

		swarm.listen(port);
		swarm.join(db.key.toString("hex"));

		swarm.on("connection", (conn, info) => {
			handlers.onConnect(info);

			var r = db.replicate({ live: false });
			r.on("data", () => console.log("r"));
			conn.on("data", () => console.log("c"));

			r.pipe(conn).pipe(r);

			r.on("error", err => console.error("error", err));
			r.on("end", () => console.log("end"));
		});
	});

	return new Promise(done => {});
});
