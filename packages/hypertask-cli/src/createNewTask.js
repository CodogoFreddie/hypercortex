import { createObj, setObj } from "@hypercortex/wrapper";
import * as R from "ramda";

import easyTypeId from "./easyTypeId";
import parseDateTime, { parseRecur } from "./parseDateTime";

const categoriseArgs = R.cond([
	[
		R.test(/^\+\w+/),
		R.pipe(
			R.replace("+", ""),
			R.objOf("tag"),
		),
	],
	[
		R.test(/[\w ]+:[\w ]+/),
		R.pipe(
			R.split(":"),
			([prop, value]) => ({
				prop,
				value,
			}),
		),
	],
	[x => parseInt(x, 10), x => ({ int: parseInt(x, 10) })],
	[R.T, plain => ({ plain })],
]);

const parseAndStringifyDateTime = R.pipe(
	parseDateTime,
	x => x.toISOString(),
);

const parseModificationArgs = R.pipe(
	R.map(categoriseArgs),

	R.reduce(
		(obj, { plain, prop, value, tag }) => ({
			...obj,
			description: [obj.description, plain].filter(Boolean).join(" "),
			...(prop && {
				[prop]: value,
			}),
			tags: [...obj.tags, tag].filter(Boolean),
		}),
		{
			description: "",
			tags: [],
		},
	),

	R.evolve({
		blocked: parseAndStringifyDateTime,
		due: parseAndStringifyDateTime,
		start: parseAndStringifyDateTime,
		stop: parseAndStringifyDateTime,
		wait: parseAndStringifyDateTime,
		recur: parseRecur,
	}),
);

const createNewTask = async (db, args) => {
	const id = easyTypeId(16);
	await createObj(db, "task", id);
	const task = parseModificationArgs(args);

	await setObj(db, "task", id, task);
};

export default createNewTask;
