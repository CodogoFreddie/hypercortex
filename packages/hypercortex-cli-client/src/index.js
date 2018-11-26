#!env node

import path from "path";
import hyperswarm from "@hyperswarm/network";
import net from "net";
import { spawn } from "child_process";

import { getAPort, createStateHandlers } from "./util";
import getADb from "./getADb";

// if this module is called as an executable, startup a new hypercortex mirroring server that forms part of an always on mesh network to replicate the hypercortex
export const main = async () => {
	const [_, __, key] = process.argv;
	const db = await getADb("server", key);

	const localPort = await getAPort();

	net.createServer(socket => {
		try {
			//console.log("server", "got a connection");
			const stream = db.replicate({ live: false });
			stream.pipe(socket).pipe(stream);

			//[socket, stream].map(x =>
			//["error", "end"].map(event =>
			//x.on(event, e => console.log("server", event, e)),
			//),
			//);
		} catch (e) {
			//console.log("server", "there was a problem");
		}
	}).listen(localPort);

	await createStateHandlers("server").setState({
		localPort,
	});

	//console.log(`sharing cortex on local port ${localPort}`);

	const swarm = hyperswarm({ ephemeral: false });
	swarm.join(db.discoveryKey, {
		lookup: true,
		announce: true,
	});
	swarm.on("connection", (socket, details) => {
		//console.log("new connection!", details);
		const stream = db.replicate({ live: true });
		stream.pipe(socket).pipe(stream);
	});

	//console.log("sharing cortex to global swarm");
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
				//console.log("here", e);
				fail(e);
			}
		});
	} catch (e) {
		//console.log("no local server detected: starting one now!");
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

	return db;
};

export default dbKey;
