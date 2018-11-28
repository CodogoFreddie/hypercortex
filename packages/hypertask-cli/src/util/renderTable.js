import * as R from "ramda";
import { format, toDate, formatDistanceWithOptions } from "date-fns/fp";

import createTableRenderer from "@hypercortex/render-table";
import generateUniqPrefixes from "@hypercortex/uniq-prefixes";

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
	snooze: formatDateTime,
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
	"tags",
	"recur",
	"snooze",
]);

const renderTable = R.pipe(
	R.filter(
		R.complement(
			R.either(
				R.prop("done"),
				({ wait }) => wait > new Date().toISOString(),
			),
		),
	),

	R.map(
		R.pipe(
			R.toPairs,
			R.filter(([_, val]) => typeof val !== "undefined"),
			R.fromPairs,
		),
	),

	tasks => {
		const ids = generateUniqPrefixes(tasks.map(R.prop("id")));

		return tasks.map(({ id, ...rest }) => ({
			id,
			...rest,
			key: ids[id],
		}));
	},

	R.map(formatTask),
	hyperTaskTableify,
	console.log,
);

export default renderTable;
