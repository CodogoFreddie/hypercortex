import * as R from "ramda";

import applyModificationsToObj from "../util/applyModificationsToObj";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";
import renderTable from "../util/renderTable";

const modify = async ({ filter, modifications, taskAll, task }) => {
	const deleteObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	const deleteIds = await Promise.all(deleteObjs.map(t => t.idGet()));
	const deleteIdsSet = new Set(deleteIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(
			R.when(({ id }) => deleteIdsSet.has(id), R.assoc("textColor", 1)),
		),
		renderTable,
	)(taskObjs);

	await Promise.all(deleteObjs.map(obj => obj.delete()));
};

export default modify;
