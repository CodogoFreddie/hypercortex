import * as R from "ramda";

const createToObjectHandler = async (
	id,
	scalars,
	collections,
	singleRelations,
	obj,
	depth = 0,
) => {
	const pairs = await Promise.all([
		...[...scalars, "score"].map(key =>
			obj[`${key}Get`]().then(value => [key, value]),
		),

		...collections.map(collection => {
			const key = collection.name || collection;
			return obj[`${key}Get`]().then(value => [key, value]);
		}),

		...singleRelations.map(({ name }) => {
			const key = name;
			return obj[`${key}Get`]().then(subType => {
				if (!subType) {
					return [key, null];
				}

				if (depth > 1) {
					return subType
						.toJsObject(depth - 1)
						.then(value => [key, value]);
				}

				const value = subType;
				return [key, value];
			});
		}),
	]);

	return {
		id,
		...R.fromPairs(pairs),
	};
};

export default createToObjectHandler;
