import * as R from "ramda";

import resolveNodeConflict from "./resolveNodeConflict";

import createScalarHandlers from "./createScalarHandlers";
import createCollectionHandlers from "./createCollectionHandlers";
import createSingleRelation from "./createSingleRelation";
import createGetAllOfObject from "./createGetAllOfObject";

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
				scoreGet: () => calculateScore(getObject(id)),
			},
			createScalarHandlers(type, scalars, db, id),
			createCollectionHandlers(type, collections, db, id),
			createSingleRelation(type, one, db, id),
		);
	return {
		[type]: getObject,
		[`${type}All`]: createGetAllOfObject(type, db, getObject),
	};
};

export default createObjecTypeWrapper;
