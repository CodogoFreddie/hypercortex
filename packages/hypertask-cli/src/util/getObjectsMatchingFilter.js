import * as R from "ramda";

const getObjectsMatchingFilter = async (
	taskAll,
	task,
	filter,
	safe = false,
) => {
	const ids = R.pipe(
		R.find(R.path(["prop", "description"])),
		R.path(["prop", "description"]),
		R.defaultTo(""),
		R.split(" "),
		R.filter(R.length),
		x => new Set(x),
	)(filter);

	const filterTags = R.pipe(
		R.filter(R.prop("plus")),
		R.map(R.prop("plus")),
	)(filter);

	const tasks = await taskAll();

	if (ids.size === 0 && filterTags.length === 0 && safe) {
		return tasks;
	}

	const taskObjs = await Promise.all(
		tasks.map(t =>
			Promise.all([t.idGet(), t.tagsGet()]).then(([id, tags]) => {
				if (
					ids.has(id) ||
					R.intersection(tags, filterTags).length > 0
				) {
					return t;
				} else {
					return false;
				}
			}),
		),
	).then(R.filter(Boolean));

	return taskObjs;
};

export default getObjectsMatchingFilter;
