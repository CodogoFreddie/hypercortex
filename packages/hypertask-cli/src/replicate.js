import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";
import discovery from "discovery-swarm";
import swarmDefaults from "dat-swarm-defaults";
import openport from "openport";

const shortDisplayId = id => `${id.slice(0, 5)}...${id.slice(-3)}`;

const replicate = (db, until) => {
	const hasAlreadyConnectedTo = {};

	return new Promise((done, fail) => {
		openport.find({ startingPort: 15423 }, (err, port) => {
			console.log(
				`replicating ${shortDisplayId(
					db.key.toString("hex"),
				)} on port ${port}`,
			);

			var swarm = discovery(
				swarmDefaults({
					id: db.local.key,
				}),
			);

			swarm.listen(port);
			swarm.join(db.key.toString("hex"));

			swarm.on("connection", (conn, info) => {
				const remoteKey = info.id.toString("hex");
				//if (!hasAlreadyConnectedTo[remoteKey]) {
				hasAlreadyConnectedTo[remoteKey] = true;
				const live = !until;
				const timeout = 0;
				const ack = true;

				var r = db.replicate({ live, timeout, ack });
				conn.pipe(r).pipe(conn);

				console.log(
					`connected to         ${shortDisplayId(remoteKey)} @ ${
						info.host
					}:${info.port} (live: ${live})`,
				);

				conn.on("error", () => {
					console.log(
						`error with           ${shortDisplayId(
							info.id.toString("hex"),
						)}`,
					);
					console.log({
						until,
						remoteKey,
					});
					if (until === remoteKey) {
						fail();
					}
				});
				conn.on("end", () => {
					console.log(
						`end with             ${shortDisplayId(
							info.id.toString("hex"),
						)}`,
					);
					if (until === remoteKey) {
						done();
					}
				});
			});
		});
	});
};

export default replicate;
