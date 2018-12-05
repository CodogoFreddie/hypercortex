import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";
import applyRecur from "../util/applyRecur";

const done = async ({ filter, modifications, taskAll, task }) => {
	const doneObjs = await getObjectsMatchingFilter(taskAll, task, filter);

	const doneIds = await Promise.all(doneObjs.map(t => t.idGet()));
	const doneIdsSet = new Set(doneIds);

	const tasks = await taskAll();
	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	R.pipe(
		R.map(R.when(({ id }) => doneIdsSet.has(id), R.assoc("textColor", 1))),
		renderTable,
	)(taskObjs);

	await Promise.all(
		doneObjs.map(async t => {
			const [due, wait, recur] = await Promise.all([
				t.dueGet(),
				t.waitGet(),
				t.recurGet(),
			]);

			if (recur) {
				const recurer = applyRecur(recur);
				if (wait < due) {
					return Promise.all([
						t.dueSet(recurer(due)),
						t.waitSet(recurer(wait)),
					]);
				} else {
					return t.dueSet(recurer(due));
				}
			} else {
				return t.doneSet(new Date().toISOString());
			}
		}),
	);
};

export default done;
