import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import parseDateTimeShortcut from "../util/parseDateTimeShortcut";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";

const dateTimeProps = new Set(["due", "wait", "sleep", "snooze"]);

const modify = async ({ filter, modifications, taskAll, task }) => {
	const modifyObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	console.log(modifications);

	for (const task of modifyObjs) {
		for (const { prop, plus, minus } of modifications) {
			if (prop) {
				const [key] = R.keys(prop);
				const [value] = R.values(prop);

				if (key === "description" && value.length === 0) {
					continue;
				}

				if (value === null) {
					await task[`${key}Set`](undefined);
				} else {
					if (dateTimeProps.has(key)) {
						await task[`${key}Set`](parseDateTimeShortcut(value));
					} else {
						await task[`${key}Set`](value);
					}
				}
			}

			if (plus) {
				await task.tagsAdd(plus);
			}

			if (minus) {
				await task.tagsRemove(minus);
			}
		}
	}

	const modifyIds = await Promise.all(modifyObjs.map(t => t.idGet()));
	const modifyIdsSet = new Set(modifyIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(
			R.when(({ id }) => modifyIdsSet.has(id), R.assoc("textColor", 4)),
		),
		renderTable,
	)(taskObjs);
};

export default modify;
