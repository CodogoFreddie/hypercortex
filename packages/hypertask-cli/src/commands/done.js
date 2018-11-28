import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import parseDateTimeShortcut from "../util/parseDateTimeShortcut";
import findIdsFromFilter from "../util/findIdsFromFilter";

const done = async ({ filter, modifications, taskAll, task }) => {
	const newID = getId(16);
	const newTask = task(newID);

	const allTasks = await taskAll();

	const allObjects = await Promise.all(allTasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(R.when(R.propEq("id", newID), R.assoc("textColor", 2))),
		renderTable,
	)(allObjects);
};

export default done;
