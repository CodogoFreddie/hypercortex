import * as R from "ramda";

import profilePromise from "@hypercortex/profile-timestamp";

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";

const basicDisplay = async ({ filter, modifications, taskAll, task }) => {
	const tasks = await profilePromise("getObjectsMatchingFilter", getObjectsMatchingFilter(taskAll, task, filter, true));

	const taskObjs = await profilePromise("mapTasksToObjects", Promise.all(tasks.map(t => t.toJsObject())));

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
};

export default basicDisplay;
