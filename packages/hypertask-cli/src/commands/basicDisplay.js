import * as R from "ramda";

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";
import logTime from "../util/profileTime";

const basicDisplay = async ({ filter, modifications, taskAll, task }) => {
	logTime("basicDisplay", "start")
	const tasks = await getObjectsMatchingFilter(taskAll, task, filter, true);
	logTime("basicDisplay", "tasks", "got")

	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));
	logTime("basicDisplay", "tasks", "mappedToObjects")

	const nowString = new Date().toISOString();

	R.pipe(
		R.map(
			R.when(
				({ due }) => due && due < nowString,
				R.assoc("textColor", 5),
			),
		),
		renderTable,
	)(taskObjs);
	logTime("basicDisplay", "printed")
};

export default basicDisplay;
