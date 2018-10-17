import * as R from "ramda";
import { format, toDate, formatDistanceWithOptions } from "date-fns/fp";

import { getObjs, getObj } from "@hypercortex/wrapper";
import createTableRenderer from "@hypercortex/render-table";
import generateUniqPrefixes from "@hypercortex/uniq-prefixes";

import addScoreToTask from "./addScoreToTask";

const defaultifyTask = R.pipe(R.over(R.lensProp("tags"), R.defaultTo([])));

const exportifyTask = R.pipe(
	R.evolve({
		tags: R.pipe(
			R.map(x => `+${x}`),
			R.join(" "),
		),
		wait: wait => `wait:${wait}`,
		due: due => `due:${due}`,
		priority: priority => `priority:${priority}`,
		recur: ({ n, period }) => `recur:${n + period}`,
	}),

	R.pick(["description", "tags", "wait", "due", "priority", "recur"]),

	R.values,

	R.prepend("task add"),

	R.join(" "),
);

const exportTasks = async db => {
	const tasks = {};
	const getTask = getObj(db, "task");

	for await (const id of getObjs(db, "task")) {
		const rawTask = await getTask(id);

		tasks[id] = R.pipe(
			defaultifyTask,
			addScoreToTask,
		)(rawTask);
	}

	const ids = generateUniqPrefixes(R.keys(tasks));

	for (const id in ids) {
		tasks[id].key = ids[id];
	}

	const tasksSorted = R.pipe(
		R.values,
		R.reject(R.prop("done")),
		R.sort(
			R.descend(
				R.pipe(
					R.prop("score"),
					Number,
				),
			),
		),
		R.map(exportifyTask),
		R.append("\n"),
		R.prepend("\n"),
		R.join("\n"),
	)(tasks);

	console.log(tasksSorted);
};

export default exportTasks;
