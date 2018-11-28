import * as R from "ramda";
import { addHours } from "date-fns/fp";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";

const snooze = async ({ filter, modifications, taskAll, task }) => {
	const snoozeObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	const snoozeIds = await Promise.all(snoozeObjs.map(t => t.idGet()));
	const snoozeIdsSet = new Set(snoozeIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(
			R.when(({ id }) => snoozeIdsSet.has(id), R.assoc("textColor", 4)),
		),
		renderTable,
	)(taskObjs);

	await Promise.all(
		snoozeObjs.map(t => t.snoozeSet(addHours(1, new Date()).toISOString())),
	);
};

export default snooze;
