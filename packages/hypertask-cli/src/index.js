import hyperdb from "hyperdb";
import envpaths from "env-paths";
import * as R from "ramda";

import { readyGate, setObj } from "@hypercortex/wrapper";

import renderTable from "./renderTable";
import createNewTask from "./createNewTask";

console.log(envpaths("hypertask").data);
const db = hyperdb(envpaths("hypertask").data, { valueEncoding: "json" });

const commands = new Set(["add"]);

const partitionCommandsAndArgs = R.pipe(
	R.slice(2, Infinity),
	R.partition(x => commands.has(x)),
);

const main = async db => {
	await readyGate(db);

	console.log(`tasks for hypercortex://${db.key.toString("hex")} \n`);

	const [[command], args] = partitionCommandsAndArgs(process.argv);

	if (!command) {
		await renderTable(db);
		return;
	}

	switch (command) {
		case "add": {
			await createNewTask(db, args);
			await renderTable(db);
			break;
		}
	}
};

main(db);
