import hyperdb from "hyperdb";
import envpaths from "env-paths";

import { readyGate, setObj } from "@hypercortex/wrapper";

import renderTable from "./renderTable";
import createNewTask from "./createNewTask";

const db = hyperdb(envpaths("hypertask").data, { valueEncoding: "json" });

const main = async db => {
	await readyGate(db);

	console.log(`tasks for hypercortex://${db.key.toString("hex")} \n`);

	const command = process.argv[2];

	if (!command) {
		await renderTable(db);
		return;
	}

	console.log({ command });

	switch (command) {
		case "add": {
			await createNewTask(db);
			await renderTable(db);
			break;
		}

		default: {
			console.log(`${command} is not a valid command`);
			break;
		}
	}
};

main(db);
