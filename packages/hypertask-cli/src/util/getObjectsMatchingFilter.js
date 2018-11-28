import * as R from "ramda";

const getObjectsMatchingFilter = async (taskAll, task, filter) => {
	const ids = R.pipe(
		R.find(R.path(["prop", "description"])),
		R.path(["prop", "description"]),
		R.split(" "),
	)(filter);

	const tasks = await taskAll();

	const taskObjs = await Promise.all(
		tasks.map(t =>
			Promise.all([t.idGet()]).then(([id]) => ({
				id,
			})),
		),
	);

	return taskObjs
		.filter(t => ids.some(id => t.id.startsWith(id)))
		.map(R.prop("id"))
		.map(task);
};

export default getObjectsMatchingFilter;
