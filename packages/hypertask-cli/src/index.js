import * as R from "ramda";
import os from "os";

import getCortexDb from "@hypercortex/cli-get-db";
import createTask from "@hypercortex/object-type-task";
import createTelemetry from "@hypercortex/object-type-telemetry";

import add from "./commands/add";
import basicDisplay from "./commands/basicDisplay";
import done from "./commands/done";
import hyper from "./commands/hyper";
import modify from "./commands/modify";
import snooze from "./commands/snooze";
import share from "./commands/share";
import deleteCommand from "./commands/delete";
import exportCommand from "./commands/export"

import partitionCommandsAndArgs from "./util/parseArgs";
import logTime from "./util/profileTime";

const commandToFunction = {
	add,
	hyper,
	done,
	modify,
	snooze,
	share,
	export: exportCommand,
	delete: deleteCommand,
};

const main = async () => {
	logTime("start");
	const db = await getCortexDb();
	logTime("gotDb");

	console.log(`cortex: "${db.key.toString("hex")}"
local:  "${db.local.key.toString("hex")}"`);

	const { task, taskAll } = createTask(db);
	const { telemetry } = createTelemetry(db);

	telemetry(db.local.key.toString("hex")).nameSet(os.hostname());

	const { filter, command, modifications } = partitionCommandsAndArgs(
		commandToFunction,
	)(process.argv);

	logTime("startCommand");
	await (commandToFunction[command] || basicDisplay)({
		filter,
		modifications,
		taskAll,
		task,
		db,
	});
	logTime("end");
};

try {
	main();
} catch (e) {
	console.error(e);
}
