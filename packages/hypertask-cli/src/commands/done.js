import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";

const done = async ({ filter, modifications, taskAll, task }) => {
	const doneObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	const doneIds = await Promise.all(doneObjs.map(t => t.idGet()));
	const doneIdsSet = new Set(doneIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(R.when(({ id }) => doneIdsSet.has(id), R.assoc("textColor", 1))),
		renderTable,
	)(taskObjs);

	await Promise.all(doneObjs.map(t => t.doneSet(new Date().toISOString())));
};

export default done;
