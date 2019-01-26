import * as R from "ramda";

import resolveNodeConflict from "./resolveNodeConflict";

import createScalarHandlers from "./createScalarHandlers";
import createCollectionHandlers from "./createCollectionHandlers";
import createSingleRelation from "./createSingleRelation";
import createGetAllOfObject from "./createGetAllOfObject";
import createToObjectHandler from "./createToObjectHandler";
import createDeleteHandler from "./createDeleteHandler";

const calculateScoreDefault = () => 1;
const createObjecTypeWrapper = ({
	type,
	calculateScore = calculateScoreDefault,
	properties: { scalars = [], collections = [] } = {},
	relations: { one = [], many = [] } = {},
}) => db => {
	const getObject = id => {
		const obj = Object.assign(
			{
				toJsObject: depth => {
					return createToObjectHandler(
						id,
						scalars,
						collections,
						one,
						obj,
						depth,
					);
				},
				fromJsObject: obj => Promise.resolve(),
				idGet: () => id,
				typeGet: () => type,
				scoreGet: () => Promise.resolve(calculateScore(obj)),
			},
			createDeleteHandler(type, db, id),
			createScalarHandlers(type, scalars, db, id),
			createCollectionHandlers(type, collections, db, id),
			createSingleRelation(type, one, db, id),
		);
		return obj;
	};

	return {
		[type]: getObject,
		[`${type}All`]: createGetAllOfObject(type, db, getObject),
	};
};

export default createObjecTypeWrapper;
