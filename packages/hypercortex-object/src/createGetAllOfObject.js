import * as R from "ramda";

import resolveNodeConflict from "./resolveNodeConflict";

const sortObjects = async objs => {
	const objsWithScores = await Promise.all(
		objs.map(obj =>
			obj.scoreGet().then(score => ({
				obj,
				score,
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
			err
				? fail(err)
				: sortObjects(
						dat.map(
							R.pipe(
								resolveNodeConflict,
								R.prop("key"),
								R.replace(`data/${type}/`, ""),
								R.replace(/\/.+/, ""),
								getObject,
							),
						),
				  ).then(done);
		});
	});
};

export default createGetAllOfObject;
