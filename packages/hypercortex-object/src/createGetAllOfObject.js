import * as R from "ramda";

const sortObjects = async objs => {
	const objsWithScores = await Promise.all(
		objs.map(obj =>
			obj.scoreGet().then(score => ({
				obj,
				score: Number.isNaN(score) ? Number.POSITIVE_INFINITY : score,
			})),
		),
	);

	return R.pipe(
		R.sort(R.descend(R.prop("score"))),
		R.map(R.prop("obj")),
	)(objsWithScores);
};

const createGetAllOfObject = (type, db, getObject) => () => {
	return new Promise((done, fail) => {
		db.list(`data/${type}/`, { recursive: false }, (err, dat) => {
			if (err) {
				return fail(err);
			}

			R.pipe(
				R.map(
					R.pipe(
						R.prop("key"),
						R.replace(`data/${type}/`, ""),
						R.replace(/\/.+/, ""),
					),
				),
				R.uniq,
				R.map(getObject),
				sortObjects,
				done,
			)(dat);
		});
	});
};

export default createGetAllOfObject;
