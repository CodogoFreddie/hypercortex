import * as R from "ramda";

const addScoreToTask = R.pipe(
	R.assoc("score", 0),
	fromAge,
	fromPriority,
	fromDue,
	fromTimelyOverDue,
	fromUrgent,
);

export default addScoreToTask;
