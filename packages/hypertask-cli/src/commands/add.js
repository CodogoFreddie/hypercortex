import * as R from "ramda";

import getId from "@hypercortex/easy-type-id";

const add = async ({ filter, modifications, taskAll, task }) => {
	const newTask = task(getId(16));

	for (const { prop, plus, minus } of modifications) {
		if (prop) {
			const [key] = R.keys(prop);
			const [value] = R.values(prop);
			console.log({ key, value, prop });
			await newTask[`${key}Set`](value);
		}

		if (plus) {
			await newTask.tagsAdd(plus);
		}

		if (minus) {
			await newTask.tagsRemove(minus);
		}
	}

	const obj = await newTask.toJsObject();

	console.log(obj);

	const allTasks = await taskAll();

	const allObjects = await Promise.all(allTasks.map(t => t.toJsObject()));

	console.log(allObjects);
};

export default add;
