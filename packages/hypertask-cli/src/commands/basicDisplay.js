import * as R from "ramda";

import renderTable from "../util/renderTable";

const basicDisplay = async ({ filter, modifications, taskAll, task }) => {
	const tasks = await taskAll();

	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	renderTable(taskObjs);
};

export default basicDisplay;
