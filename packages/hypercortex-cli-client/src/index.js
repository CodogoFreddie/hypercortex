#!env node

import fs from "fs";
import path from "path";
import hyperdb from "hyperdb";
import envpaths from "env-paths";
import hyperswarm from "@hyperswarm/network";
import net from "net";
import { spawn } from "child_process";
import mkdirp from "mkdirp";
import lockfile from "proper-lockfile";
import { promisify } from "util";

import { rename, stat, getAPort, readyGate, createStateHandlers } from "./util";

const mkdirpp = promisify(mkdirp);

const getADb = async (type, key) => {
	process.on(
		"exit",
		() => key && console.log(type, `closing hypercortex://${key}`),
	);

	const {
		data: dataFolderPath,
		config: configFolderPath,
		temp: tempFilePath,
	} = envpaths(`hypercortex-${type}`);

	console.log(type, `loading config from ${configFolderPath}`);

	const { getState, setState } = createStateHandlers(type);

	const config = await getState();

	if (!key) {
		console.log(type, "no key provided");
		if (config.lastUsedCortex) {
			console.log(
				type,
				`last used cortex was hypercortex://${
					config.lastUsedCortex
				}, opening`,
			);
			return getADb(type, config.lastUsedCortex);
		} else {
			console.log(type, "no previously used cortex, creating a new one");

			const db = hyperdb(tempFilePath, { valueEncoding: "json" });

			await readyGate(db);

			const key = db.key.toString("hex");
			const perminantFolder = path.join(dataFolderPath, key);

			await mkdirp(dataFolderPath);
			await rename(tempFilePath, perminantFolder);

			await setState({
				lastUsedCortex: key,
			});

			console.log(type, `created hypercortex://${key}`);
			console.log(type, "opening");

			return getADb(type, key);
		}
	}

	const dbPath = path.join(dataFolderPath, key);

	console.log(type, `attempting to lock ${dbPath}`);

	await mkdirpp(`${dbPath}.meta`);

	try {
		lockfile.lockSync(`${dbPath}.meta`);
	} catch (e) {
		console.log(type, `${dbPath} already seems to be open`);
		console.error(type, e);
		process.exit(0);
	}

	try {
		await stat(dbPath);
		console.log(type, "cortex exists");
		console.log(type, `opening hypercortex://${key}`);
		const db = hyperdb(dbPath, {
			valueEncoding: "json",
		});

		await readyGate(db);

		return db;
	} catch (e) {
		console.log(type, "cortex does not exist");
		console.log(type, `creating hypercortex://${key}`);
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

	net.createServer(socket => {
		try {
			console.log("server", "got a connection");
			const stream = db.replicate({ live: true });
			stream.pipe(socket).pipe(stream);

			[socket, stream].map(x =>
				["error", "end"].map(event =>
					x.on(event, e => console.log("server", event, e)),
				),
			);
		} catch (e) {
			console.log("server", "there was a problem");
		}
	}).listen(localPort);

	await createStateHandlers("server").setState({
		localPort,
	});

	console.log(`sharing cortex on local port ${localPort}`);

	const swarm = hyperswarm({ ephemeral: false });
	swarm.join(db.discoveryKey, {
		lookup: true,
		announce: true,
	});
	swarm.on("connection", (socket, details) => {
		console.log("new connection!", details);
		const stream = db.replicate({ live: true });
		stream.pipe(socket).pipe(stream);
	});

	console.log("sharing cortex to global swarm");
};

//if this module is included as a submodule, it returns a hyperdb instance that will replicate with the local hypercortex server untill they're both equal
const dbKey = async key => {
	const [db, { localPort: serverLocalPort }] = await Promise.all([
		await getADb("client"),
		createStateHandlers("server").getState(),
	]);

	const client = new net.Socket();

	try {
		await new Promise((done, fail) => {
			try {
				client.on("error", fail);
				client.connect(
					serverLocalPort,
					"localhost",
					(err, dat) => (err ? fail(err) : done(dat)),
				);
			} catch (e) {
				console.log("here", e);
				fail(e);
			}
		});
	} catch (e) {
		console.log("no local server detected: starting one now!");
		//spawn a server
		const scriptName = path.join(__dirname, "..", "main.js");
		spawn("node", [scriptName, db.key.toString("hex")], {
			slient: false,
			detached: true,
			stdio: ["inherit", "inherit", "inherit"],
		}).unref();

		process.exit(0);
	}

	const stream = db.replicate({ live: false });
	stream.pipe(client).pipe(stream);

	stream.on("data", data =>
		console.log("client", "data", data.toString("base64")),
	);
	stream.on("end", end => console.log("client", "end", end));

	return db;
};

export default dbKey;
