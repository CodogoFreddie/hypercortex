import * as R from "ramda";

import applyModificationsToObj from "../util/applyModificationsToObj";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";
import renderTable from "../util/renderTable";

const modify = async ({ filter, modifications, taskAll, task }) => {
	const modifyObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	await Promise.all(
		modifyObjs.map(applyModificationsToObj(modifications, taskAll)),
	);

	const modifyIds = await Promise.all(modifyObjs.map(t => t.idGet()));
	const modifyIdsSet = new Set(modifyIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(
			R.when(({ id }) => modifyIdsSet.has(id), R.assoc("textColor", 6)),
		),
		renderTable,
	)(taskObjs);
};

export default modify;
