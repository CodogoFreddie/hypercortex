import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import applyModificationsToObj from "../util/applyModificationsToObj";

const dateTimeProps = new Set(["due", "wait", "sleep", "snooze"]);

const add = async ({ filter, modifications, taskAll, task }) => {
	const newID = getId(16);
	const newTask = task(newID);

	await applyModificationsToObj(modifications, taskAll)(newTask);

	const allTasks = await taskAll();

	const allObjects = await Promise.all(allTasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(R.when(R.propEq("id", newID), R.assoc("textColor", 2))),
		renderTable,
	)(allObjects);
};

export default add;
