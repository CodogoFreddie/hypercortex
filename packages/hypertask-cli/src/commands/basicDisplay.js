import * as R from "ramda";

import renderTable from "../util/renderTable";

const basicDisplay = async ({ filter, modifications, taskAll, task }) => {
	const tasks = await taskAll();

	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

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
