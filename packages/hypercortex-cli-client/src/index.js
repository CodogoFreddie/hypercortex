import * as R from "ramda";
import path from "path";
import hyperdb from "hyperdb";
import envpaths from "env-paths";
import hyperswarm from "@hyperswarm/network";
import net from "net";

import { rename, stat, getAPort, readyGate, createStateHandlers } from "./util";

const getADb = async (type, key) => {
	const {
		data: dataFolderPath,
		config: configFolderPath,
		temp: tempFilePath,
	} = envpaths(`hypercortex-${type}`);

	//console.log(`loading config from ${configFolderPath}`);

	const { getState, setState } = createStateHandlers(type);

	const config = await getState();

	if (!key) {
		//console.log("no key provided");
		if (config.lastUsedCortex) {
			//console.log(
			//`last used cortex was hypercortex://${
			//config.lastUsedCortex
			//}, opening`,
			//);
			return getADb(type, config.lastUsedCortex);
		} else {
			//console.log("no previously used cortex, creating a new one");

			const dbContainer = {
				db: hyperdb(tempFilePath, { valueEncoding: "json" }),
			};

			await readyGate(dbContainer.db);

			const key = dbContainer.db.key.toString("hex");
			const perminantFolder = path.join(dataFolderPath, key);
			delete dbContainer.db;

			await rename(tempFilePath, perminantFolder);

			await setState({
				lastUsedCortex: key,
			});

			//console.log(`created hypercortex://${key}`);
			//console.log("opening");

			return getADb(type, key);
		}
	}

	const dbPath = path.join(dataFolderPath, key);

	try {
		await stat(dbPath);
		//console.log("cortex exists");
		//console.log(`opening hypercortex://${key}`);
		const db = hyperdb(dbPath, {
			valueEncoding: "json",
		});

		await readyGate(db);

		return db;
	} catch (e) {
		//console.log("cortex does not exist");
		//console.log(`creating hypercortex://${key}`);
		const db = hyperdb(dbPath, key, {
			valueEncoding: "json",
		});

		await readyGate(db);

		return db;
	}
};

// if this module is called as an executable, startup a new hypercortex mirroring server that forms part of an always on mesh network to replicate the hypercortex
export const main = async () => {
	const [_, __, key] = process.argv;
	const db = await getADb("server", key);

	const localPort = await getAPort();

	console.log(`${localPort};`);

	net.createServer(socket => {
		const stream = db.replicate({ live: false });
		stream.pipe(socket).pipe(stream);
	}).listen(localPort);

	await createStateHandlers("server").setState({
		localPort,
	});

	const swarm = hyperswarm({ ephemeral: false });
	swarm.join(db.discoveryKey, {
		lookup: true,
		announce: true,
	});
	swarm.on("connection", (socket, details) => {
		console.log("new connection!", details);
		const stream = db.replicate({ live: false });
		stream.pipe(socket).pipe(stream);
	});
};

//if this module is included as a submodule, it returns a hyperdb instance that will replicate with the local hypercortex server untill they're both equal
const dbKey = async key => {
	const db = await getADb("client");
	const scriptName = path.basename(__filename);
	console.log({ scriptName });
};

export default dbKey;
