import * as R from "ramda";

const resolveNodeConflict = R.reduce(
	(l, r) => (l.value.modifiedAt > r.value.modifiedAt ? l : r),
	{ value: { modifiedAt: "" } },
);

export default resolveNodeConflict;
