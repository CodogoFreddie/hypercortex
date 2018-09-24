import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";

import { openDefaultDb, readyGate, setObj } from "@hypercortex/wrapper";

import renderTable from "./renderTable";
import createNewTask from "./createNewTask";
import modifyTasks from "./modifyTasks";
import setupHyperDb from "./setupHyperDb";

const noop = () => {};
const setPropToNow = prop => (db, _, filter) =>
	modifyTasks(db, [`${prop}:${new Date().toISOString()}`], filter);
const commandToFunction = {
	add: createNewTask,
	modify: modifyTasks,
	done: setPropToNow("done"),
	start: setPropToNow("start"),
	stop: setPropToNow("stop"),
	hyper: setupHyperDb,
};

const partitionCommandsAndArgs = R.pipe(
	R.slice(2, Infinity),
	R.splitWhen(x => commandToFunction[x]),
	([filter, [command, ...modifications]]) => ({
		filter,
		command,
		modifications,
	}),
);

const main = async () => {
	const db = await openDefaultDb("hypercortex");

	await readyGate(db);

	console.log(`tasks for hypercortex://${db.key.toString("hex")}\n`);

	const { filter, command, modifications } = partitionCommandsAndArgs(
		process.argv,
	);

	const opperation = commandToFunction[command] || noop;

	await opperation(db, modifications, filter);

	await renderTable(db);
};

try {
	main();
} catch (e) {
	console.error(e);
}
