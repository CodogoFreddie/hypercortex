import * as R from "ramda";
import { format, toDate, formatDistanceWithOptions } from "date-fns/fp";

import { getObjs, getObj } from "@hypercortex/wrapper";
import createTableRenderer from "@hypercortex/render-table";
import generateUniqPrefixes from "@hypercortex/uniq-prefixes";

import addScoreToTask from "./addScoreToTask";

const formatDateTime = R.pipe(
	toDate,
	formatDistanceWithOptions({ addSuffix: true }, new Date()),
);

const formatScore = R.pipe(
	R.multiply(100),
	x => Math.ceil(x),
	R.multiply(1 / 100),
	x => x.toPrecision(3),
);

const formatTask = R.evolve({
	done: formatDateTime,
	due: formatDateTime,
	score: formatScore,
	start: formatDateTime,
	stop: formatDateTime,
	wait: formatDateTime,
	tags: R.pipe(
		R.map(x => `+${x}`),
		R.join(" "),
	),
	recur: ({ n, period }) =>
		n +
		" " +
		{
			d: "days",
			w: "weeks",
			m: "months",
			y: "years",
		}[period],
});

const hyperTaskTableify = createTableRenderer([
	"score",
	"key",
	"description",
	"due",
	"priority",
	"tags",
	"recur",
	"start",
]);

const defaultifyTask = R.pipe(R.over(R.lensProp("tags"), R.defaultTo([])));

const renderTable = async (db, filterFn = R.identity) => {
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
		R.reject(({ wait }) => wait > new Date().toISOString()),
		R.reject(R.prop("done")),
		R.filter(filterFn),
		R.sort(
			R.descend(
				R.pipe(
					R.prop("score"),
					Number,
				),
			),
		),
		R.map(formatTask),
	)(tasks);

	const renderedString = hyperTaskTableify(tasksSorted);

	console.log(renderedString);
};

export default renderTable;
