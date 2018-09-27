import * as R from "ramda";
import crypto from "hypercore-crypto";
import envpaths from "env-paths";
import fs from "fs";
import util from "util";

import { readyGate } from "@hypercortex/wrapper";

const categoriseArgs = R.cond([
	[
		R.test(/^\+\w+/),
		R.pipe(
			R.replace("+", ""),
			R.objOf("tag"),
		),
	],
	[
		R.test(/[\w ]+:[\w ]+/),
		R.pipe(
			R.split(":"),
			([prop, value]) => ({
				prop,
				value,
			}),
		),
	],
	[x => parseInt(x, 10), x => ({ int: parseInt(x, 10) })],
	[R.T, plain => ({ plain })],
]);

const parseModificationArgs = R.pipe(
	R.map(categoriseArgs),

	R.reduce(
		(obj, { plain, prop, value, tag }) => ({
			...obj,
			description: [obj.description, plain].filter(Boolean).join(" "),
			...(prop && {
				[prop]: value,
			}),
			tags: [...obj.tags, tag].filter(Boolean),
		}),
		{
			description: "",
			tags: [],
		},
	),
);

const writeFile = util.promisify(fs.writeFile);

const setupHyperDb = async (db, modifications, filter) => {
	console.log(`your cortex is    "${db.key.toString("hex")}"`);
	console.log(`your local key is "${db.local.key.toString("hex")}"`);
	db.authorized(db.local.key, (err, authed) => {
		if (authed) {
			console.log("you are authorised to use this hypercortex");
		} else {
			console.log("you not are authorised to use this hypercortex");
		}
	});

	const { set, auth } = parseModificationArgs(modifications);

	if (set) {
		await writeFile(envpaths("hypercortex").config, set);
		return;
	}

	if (auth) {
		await new Promise(done => db.authorize(Buffer.from(auth, "hex"), done));

		const authed = await new Promise((done, fail) =>
			db.authorized(Buffer.from(auth, "hex"), (err, auth) => {
				err ? fail(err) : done(auth);
			}),
		);

		if (authed) {
			console.log(`authorised "${auth}"`);
		}

		return;
	}
};

export default setupHyperDb;
