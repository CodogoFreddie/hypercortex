import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";
import addHours from "date-fns/fp/addHours";

import { justReplicate, openDefaultDb, readyGate } from "@hypercortex/wrapper";

import renderTable from "./renderTable";
import createNewTask from "./createNewTask";
import modifyTasks from "./modifyTasks";
import setupHyperDb from "./setupHyperDb";
import markAsDone from "./markAsDone";
import openDb from "./openDb";
import replicate from "./replicate";
import authOtherWriter from "./authOtherWriter";
import addTelemetry from "./addTelemetry";
import exportTasks from "./exportTasks";

const noop = () => {};
const setPropToTime = (prop, time = new Date()) => (db, _, filter) =>
	modifyTasks(db, [`${prop}:${time.toISOString()}`], filter);
const commandToFunction = {
	add: createNewTask,
	modify: modifyTasks,
	done: markAsDone,
	start: setPropToTime("start"),
	stop: setPropToTime("stop"),
	snooze: setPropToTime("wait", addHours(1, new Date())),
	hyper: setupHyperDb,
	export: exportTasks,
	share: db => replicate(db),
	auth: authOtherWriter,
};

const partitionCommandsAndArgs = R.pipe(
	R.slice(2, Infinity),
	R.splitWhen(x => commandToFunction[x]),
	R.when(R.pathEq([1, "length"], 0), R.prepend([])),
	([filter, [command, ...modifications]]) => ({
		filter,
		command,
		modifications,
	}),
);

const main = async () => {
	const db = await openDb();

	await addTelemetry(db);

	console.log(`loaded hypercortex:   "${db.key.toString("hex")}"`);
	console.log(`with local key:       "${db.local.key.toString("hex")}"`);

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
