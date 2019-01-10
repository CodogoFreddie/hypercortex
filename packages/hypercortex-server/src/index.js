import envPaths from "env-paths";
import hyperdb from "hyperdb";
import hyperswarm from "@hyperswarm/network";
import net from "net";
import openport from "openport";
import path from "path";
import { spawn } from "child_process";
import winston from "winston";
import forever from "forever-monitor";

const DEFAULT_PORT = 51412;
const getAPort = () =>
	new Promise((done, fail) => {
		openport.find({ startingPort: DEFAULT_PORT }, (err, port) =>
			err ? fail(err) : done(port),
		);
	});

const { data: dataPath, config: configPath, log: logPath } = envPaths(
	"hypercortex-server",
);

const createLogger = () => {
	const logger = winston.createLogger({
		level: "info",
		format: winston.format.json(),
		transports: [new winston.transports.File({ filename: logPath })],
	});

	if (process.env.NODE_ENV !== "production") {
		logger.add(
			new winston.transports.Console({
				format: winston.format.simple(),
			}),
		);
	}

	return logger;
};

const logger = createLogger();

export const connect = async key => {
	const client = new net.Socket();

	console.log(`cli:  trying to get a connection for ${key}`);

	try {
		await new Promise((done, fail) => {
			client.on("error", fail);
			client.connect(
				DEFAULT_PORT,
				"localhost",
				(err, dat) => (err ? fail(err) : done(dat)),
			);
		});
	} catch (e) {
		console.error(
			"cli:  there doesn't seem to be a server running, trying to start one now",
		);

		const scriptName = path.join(__dirname, "..", "main.js");
		spawn("node", [scriptName], {
			detached: true,
		});

		await new Promise(done => setTimeout(done, 1000));

		return connect(key);
	}

	const port = await new Promise((done, fail) => {
		client.on("error", fail);
		client.on("data", buf => done(parseInt(buf.toString(), 10)));
		client.write(key);
	});

	client.end();

	const replicator = new net.Socket();

	await new Promise((done, fail) => {
		replicator.on("error", fail);
		replicator.connect(
			port,
			"localhost",
			(err, dat) => (err ? fail(err) : done(dat)),
		);
	});

	return replicator;
};

const dbs = {};
const getDb = async key => {
	if (!dbs[key]) {
		logger.info(`not currently hosting ${key}, creating now`);

		const db = hyperdb(path.join(dataPath, key), Buffer.from(key, "hex"), {
			valueEncoding: "json",
		});

		await new Promise(done => db.on("ready", done));

		logger.info(`created ${key}`);
		dbs[key] = db;

		const swarm = hyperswarm({ ephemeral: false });
		swarm.join(db.discoveryKey, {
			lookup: true,
			announce: true,
		});
		swarm.on("connection", (socket, details) => {
			logger.info(`connected to peer in swarm for ${key}`);

			const stream = db.replicate({ live: true });
			stream.pipe(socket).pipe(stream);
		});
	}

	return dbs[key];
};

const onConnecitonToAnnounceServer = announceSocket => {
	logger.info("incoming announce connection");
	try {
		announceSocket.on("error", logger.error);
		announceSocket.on("data", async data => {
			const key = data.toString();
			logger.info(`recieved request to replicated ${key}`);

			const [db, instancePort] = await Promise.all([
				getDb(key),
				getAPort(),
			]);

			logger.info(`setting up a replcation server for ${key}`);
			const replicationServer = net
				.createServer(replicationSocket => {
					logger.info(`recieved replication connection for ${key}`);
					const stream = db.replicate({ live: false });
					stream.pipe(replicationSocket).pipe(stream);
					replicationSocket.on("end", () =>
						replicationServer.close(),
					);
					replicationSocket.on("error", () =>
						replicationServer.close(),
					);
				})
				.listen(instancePort, () => {
					logger.info(`sharing ${key} @ localhost:${instancePort}`);
				});

			announceSocket.write(String(instancePort));
		});
	} catch (e) {
		logger.error(e);
	}
};

export const main = async () => {
	logger.info(`logging to ${logPath}`);

	net.createServer(onConnecitonToAnnounceServer).listen(DEFAULT_PORT, () => {
		logger.info(`created anounce server at port ${DEFAULT_PORT}`);
	});
};
