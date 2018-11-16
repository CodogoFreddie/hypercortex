import * as R from "ramda";

import createScalarHandlers from "./createScalarHandlers";
import createCollectionHandlers from "./createCollectionHandlers";

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
				fromObject: obj => Promise.resolve(),
			},
			...createScalarHandlers(type, scalars, db, id),
			...createCollectionHandlers(type, collections, db, id),
		);
	return {
		[type]: getObject,
		[`${type}All`]: () => {
			//build an index of scores, so that we can quickly return a sorted list"
			return [];
		},
	};
};

export default createObjecTypeWrapper;
