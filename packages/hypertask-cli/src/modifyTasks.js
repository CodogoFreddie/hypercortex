import * as R from "ramda";
import { getObjs, setObj } from "@hypercortex/wrapper";

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

const modifyTasks = async (db, modifications, filter) => {
	if (!filter.length) {
		return console.log("must provide a filter for modification");
	}

	const newProps = parseModificationArgs(modifications);
	if (newProps.description === "") {
		delete newProps.description;
	}
	if (Array.isArray(newProps.tags) && newProps.tags.length === 0) {
		delete newProps.tags;
	}

	for await (const id of getObjs(db, "task")) {
		if (filter.some(prefix => id.startsWith(prefix))) {
			setObj(db, "task", id, newProps);
		}
	}
};

export default modifyTasks;
