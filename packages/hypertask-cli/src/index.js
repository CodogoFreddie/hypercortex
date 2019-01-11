import * as R from "ramda";
import os from "os";

import getCortexDb from "@hypercortex/cli-get-db";
import createTask from "@hypercortex/object-type-task";
import createTelemetry from "@hypercortex/object-type-telemetry";
import { connect } from "@hypercortex/hypercortex-server";

import add from "./commands/add";
import basicDisplay from "./commands/basicDisplay";
import done from "./commands/done";
import hyper from "./commands/hyper";
import modify from "./commands/modify";
import partitionCommandsAndArgs from "./util/parseArgs";
import snooze from "./commands/snooze";

const commandToFunction = { add, hyper, done, modify, snooze };

const main = async () => {
	const db = await getCortexDb();

	const rStream = db.replicate({ live: false });

	await new Promise((done, fail) =>
		db.put(
			db.local.key.toString("hex") + "/lastAccesed",
			new Date().toISOString(),
			(err, dat) => (err ? fail(err) : done(dat)),
		),
	);

	const socket = await connect(db.key.toString("hex"));

	socket.pipe(rStream).pipe(socket);

	console.log(`cortex: "${db.key.toString("hex")}"
local:  "${db.local.key.toString("hex")}"`);

	const { task, taskAll } = createTask(db);
	const { telemetry } = createTelemetry(db);

	telemetry(db.local.key.toString("hex")).nameSet(os.hostname());

	const { filter, command, modifications } = partitionCommandsAndArgs(
		commandToFunction,
	)(process.argv);

	await (commandToFunction[command] || basicDisplay)({
		filter,
		modifications,
		taskAll,
		task,
		db,
	});
};

try {
	main();
} catch (e) {
	console.error(e);
}
