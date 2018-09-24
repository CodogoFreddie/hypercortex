import * as R from "ramda";
import crypto from "hypercore-crypto";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";

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

		R.concat([
			{
				type: "put",
				key: `data/${type}/${id}/modifiedAt`,
				value: new Date().toISOString(),
			},

			{
				type: "put",
				key: `data/${type}/${id}/modifiedBy`,
				value: db.local.key.toString("hex"),
			},
		]),
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

	for (const [{ key }] of objs) {
		yield key.split("/")[2];
	}
});
