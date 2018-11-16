import * as R from "ramda";

import resolveNodeConflict from "./resolveNodeConflict";

import createScalarHandlers from "./createScalarHandlers";
import createCollectionHandlers from "./createCollectionHandlers";
import createSingleRelation from "./createSingleRelation";

const calculateScoreDefault = () => 1;
const createObjecTypeWrapper = ({
	type,
	calculateScore = calculateScoreDefault,
	properties: { scalars = [], collections = [] } = {},
	relations: { one = [], many = [] } = {},
}) => db => {
	const getObject = id =>
		Object.assign(
			{
				toObj: (depth = 0) => Promise.resolve({}),
				fromObj: obj => Promise.resolve(),
				idGet: () => id,
				typeGet: () => type,
			},
			createScalarHandlers(type, scalars, db, id),
			createCollectionHandlers(type, collections, db, id),
			createSingleRelation(type, one, db, id),
		);
	return {
		[type]: getObject,
		[`${type}All`]: () => {
			return new Promise((done, fail) => {
				db.list(`data/${type}/`, { recursive: false }, (err, dat) => {
					err
						? fail(err)
						: done(
								dat.map(
									R.pipe(
										resolveNodeConflict,
										R.prop("key"),
										R.replace(`data/${type}/`, ""),
										R.replace(/\/.+/, ""),
										getObject,
									),
								),
						  );
				});
			});
		},
	};
};

export default createObjecTypeWrapper;
