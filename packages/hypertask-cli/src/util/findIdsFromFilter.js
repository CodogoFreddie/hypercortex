import * as R from "ramda";

const findIdsFromFilter = filter =>
	R.filter(({ id }) => {
		console.log({ filter });
		return true;
	});

export default findIdsFromFilter;
