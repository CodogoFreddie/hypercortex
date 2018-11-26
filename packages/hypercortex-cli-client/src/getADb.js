import path from "path";
import hyperdb from "hyperdb";
import envpaths from "env-paths";
import mkdirp from "mkdirp";
import lockfile from "proper-lockfile";
import { promisify } from "util";

import { rename, stat, getAPort, readyGate, createStateHandlers } from "./util";

const mkdirpp = promisify(mkdirp);

const getADb = async (type, key) => {
	//process.on(
	//"exit",
	//() => key && console.log(type, `closing hypercortex://${key}`),
	//);

	const {
		data: dataFolderPath,
		config: configFolderPath,
		temp: tempFilePath,
	} = envpaths(`hypercortex-${type}`);

	//console.log(type, `loading config from ${configFolderPath}`);

	const { getState, setState } = createStateHandlers(type);

	const config = await getState();

	if (!key) {
		//console.log(type, "no key provided");
		if (config.lastUsedCortex) {
			//console.log(
			//type,
			//`last used cortex was hypercortex://${
			//config.lastUsedCortex
			//}, opening`,
			//);
			return getADb(type, config.lastUsedCortex);
		} else {
			//console.log(type, "no previously used cortex, creating a new one");

			const db = hyperdb(tempFilePath, { valueEncoding: "json" });

			await readyGate(db);

			const key = db.key.toString("hex");
			const perminantFolder = path.join(dataFolderPath, key);

			await mkdirp(dataFolderPath);
			await rename(tempFilePath, perminantFolder);

			await setState({
				lastUsedCortex: key,
			});

			//console.log(type, `created hypercortex://${key}`);
			//console.log(type, "opening");

			return getADb(type, key);
		}
	}

	const dbPath = path.join(dataFolderPath, key);

	//console.log(type, `attempting to lock ${dbPath}`);

	await mkdirpp(`${dbPath}.meta`);

	try {
		lockfile.lockSync(`${dbPath}.meta`);
	} catch (e) {
		//console.log(type, `${dbPath} already seems to be open`);
		//console.error(type, e);
		process.exit(0);
	}

	try {
		await stat(dbPath);
		//console.log(type, "cortex exists");
		//console.log(type, `opening hypercortex://${key}`);
		const db = hyperdb(dbPath, {
			valueEncoding: "json",
		});

		await readyGate(db);

		return db;
	} catch (e) {
		//console.log(type, "cortex does not exist");
		//console.log(type, `creating hypercortex://${key}`);
		const db = hyperdb(dbPath, key, {
			valueEncoding: "json",
		});

		await readyGate(db);

		return db;
	}
};

export default getADb;
