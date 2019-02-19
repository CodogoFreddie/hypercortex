import whyRun from "why-is-node-running";
import * as R from "ramda";
import os from "os";

import getCortexDb from "@hypercortex/cli-get-db";
import createPin from "@hypercortex/object-type-pin";
import createTelemetry from "@hypercortex/object-type-telemetry";

import add from "./commands/add";
import archive from "./commands/archive";
import basicDisplay from "./commands/basicDisplay";
import deleteCommand from "./commands/delete";

global.fetch = require("node-fetch");

const commandToFunction = {
	add,
	basicDisplay,
	delete: deleteCommand,
	archive,
};

const main = async () => {
	const db = await getCortexDb();

	console.log(`cortex: "${db.key.toString("hex")}"
local:  "${db.local.key.toString("hex")}"`);

	const { pin, pinAll } = createPin(db);
	const { telemetry } = createTelemetry(db);

	telemetry(db.local.key.toString("hex")).nameSet(os.hostname());

	const command = process.argv.find( arg => Object.keys(commandToFunction).includes(arg) )

	await (commandToFunction[command || "basicDisplay"]({pin, pinAll}, ...process.argv.slice(2)));
};

try {
	main();
} catch (e) {
	console.error(e);
}
