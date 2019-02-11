import * as R from "ramda";
import { format } from "date-fns/fp";

const exportifyDate = name =>
	R.pipe(
		format("yyyy-MM-dd"),
		x => `${name}:${x}`,
	);

const exportify = R.pipe(
	R.omit(["score", "createdAt", "id", "modifiedAt"]),
	R.toPairs,
	R.filter(([key, value]) => value),
	R.fromPairs,
	R.evolve({
		due: exportifyDate("due"),
		wait: exportifyDate("wait"),
		recur: ({ n, period }) => `recur:${n}${period}`,
		done: exportifyDate("done"),
		snooze: exportifyDate("snooze"),
		tags: R.pipe(
			R.map(x => `+${x}`),
			R.join(" "),
		),
	}),
	R.values,
	R.join(" "),
	x => `task add ${x}`,
);

const exportCommand = async ({ filter, modifications, taskAll, task }) => {
	const tasks = await taskAll();

	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	taskObjs.map(exportify).forEach(x => console.log(x));
};

export default exportCommand;
