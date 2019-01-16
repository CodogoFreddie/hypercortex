import whyRun from "why-is-node-running";
import * as R from "ramda";
import os from "os";

import getCortexDb from "@hypercortex/cli-get-db";
import createPin from "@hypercortex/object-type-pin";
import createTelemetry from "@hypercortex/object-type-telemetry";

import add from "./commands/add";

const commandToFunction = { add, hyper, done, modify, snooze, share };

const main = async () => {
	const db = await getCortexDb();

	console.log(`cortex: "${db.key.toString("hex")}"
local:  "${db.local.key.toString("hex")}"`);

	const { pin, pinAll } = createPin(db);
	const { telemetry } = createTelemetry(db);

	telemetry(db.local.key.toString("hex")).nameSet(os.hostname());

	if(process.argv.length === 3){
		await add(process.argv[2], pin);
		return;
	} else {
		switch(process.argv[2]){
			case "archive":
				console.log(`should archive ${process.argv[3]}`);
				return;
			case "delete":
				console.log(`should delete ${process.argv[3]}`);
				return;

			default: 
				console.log(`unknown command "${(process.argv[2])}`);
				return;
		}
	}
};

try {
	main();
} catch (e) {
	console.error(e);
}
