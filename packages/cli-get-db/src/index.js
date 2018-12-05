import envPaths from "env-paths";
import { stat, rename, closeSync, openSync, writeFile, readFile } from "fs";
import hyperdb from "hyperdb";
import mkdirp from "mkdirp";
import net from "net";
import path from "path";
import { promisify } from "util";

const write = promisify(writeFile);
const read = promisify(readFile);
const renamep = promisify(rename);
const statp = promisify(stat);
const mkdirpp = promisify(mkdirp);

export const setNewKey = async key => {
	const configPath = envPaths("hypercortex-cli").config;
	await mkdirp(path.dirname(configPath));
	await write(configPath, key);
};

const getDb = async () => {
	const { config: configPath, temp: tempPath, data: dataPath } = envPaths(
		"hypercortex-cli",
	);
	try {
		const keyBuffer = await read(configPath);
		const key = keyBuffer.toString();
		if (!key) {
			const db = hyperdb(tempPath);
			await new Promise(done => db.on("ready", done));

			const key = db.key.toString("hex");

			await mkdirpp(dataPath);
			await renamep(tempPath, path.join(dataPath, key));
			await setNewKey(key);

			return getDb();
		}

		try {
			await statp(path.join(dataPath, key));
			const db = hyperdb(path.join(dataPath, key), {
				valueEncoding: "json",
			});
			await new Promise(done => db.on("ready", done));
			return db;
		} catch (e) {
			const db = hyperdb(
				path.join(dataPath, key),
				Buffer.from(key, "hex"),
				{
					valueEncoding: "json",
				},
			);
			await new Promise(done => db.on("ready", done));
			return db;
		}
	} catch (e) {
		await mkdirp(path.dirname(configPath));
		closeSync(openSync(configPath, "w"));

		return getDb();
	}
};

process.on("unhandledRejection", (reason, promise) => {
	console.log("Unhandled Rejection at:", reason.stack || reason);
	// Recommended: send the information to sentry.io
	// or whatever crash reporting service you use
});

export default getDb;
