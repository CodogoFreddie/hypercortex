import * as R from "ramda";

const findIdsFromFilter = filter =>
	R.filter(({ id }) => {
		return true;
	});

export default findIdsFromFilter;
