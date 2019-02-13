import * as R from "ramda";
import { addHours, formatDistanceWithOptions, format } from "date-fns/fp"

import renderTable from "../util/renderTable";
import getObjectsMatchingFilter from "../util/getObjectsMatchingFilter";
import calculatePresure from "../util/calculatePresure"; 

const basicDisplay = async ({ filter, modifications, taskAll, task }) => {
	const tasks = await getObjectsMatchingFilter(taskAll, task, filter, true);

	const taskObjs = await Promise.all(tasks.map(t => t.toJsObject()));

	const presure = calculatePresure(taskObjs.filter(R.complement(R.prop("done"))));

	const presureTarget = addHours(presure, new Date())

console.log(`presure: ${ formatDistanceWithOptions({}, new Date(), presureTarget)}, (${format("y-MM-d", presureTarget )})`);

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
