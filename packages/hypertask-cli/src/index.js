import * as R from "ramda";

import getCortexDb from "@hypercortex/hypercortex-cli-client";
import createTask from "@hypercortex/object-type-task";
import createTelemetry from "@hypercortex/object-type-telemetry";

import partitionCommandsAndArgs from "./parseArgs";

import add from "./commands/add";
import basicDisplay from "./commands/basicDisplay";

const commandToFunction = { add };

const main = async () => {
	const db = await getCortexDb();

	const { task, taskAll } = createTask(db);
	const { telemetry, telemetryAll } = createTelemetry(db);

	const { filter, command, modifications } = partitionCommandsAndArgs(
		commandToFunction,
	)(process.argv);

	await (commandToFunction[command] || basicDisplay)({
		filter,
		modifications,
		taskAll,
		task,
	});
};

try {
	main();
} catch (e) {
	console.error(e);
}
