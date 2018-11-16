import * as R from "ramda";

import createID from "@hypercortex/easy-type-id";

import resolveNodeConflict from "./resolveNodeConflict";
import createScalarHandlers from "./createScalarHandlers";

const createSingleRelation = (type, relations, db, id) => {
	const helpers = createScalarHandlers(
		type,
		relations.map(R.prop("name")),
		db,
		id,
	);

	return Object.assign(
		{},
		...relations.map(({ name, type, resolver }) => {
			const createSubTypeObject = resolver(db)[type];
			return {
				[`${name}Create`]: async () => {
					const id = createID();
					await helpers[`${name}Set`](id);
					return createSubTypeObject(id);
				},

				[`${name}Get`]: async () => {
					const id = await helpers[`${name}Get`]();
					return createSubTypeObject(id);
				},
			};
		}),
	);
};

export default createSingleRelation;
