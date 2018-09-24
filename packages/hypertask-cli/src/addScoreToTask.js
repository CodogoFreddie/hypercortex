import * as R from "ramda";

const modifyScore = fn => task => ({
	...task,
	score: task.score + (fn(task) || 0),
});

const fromPriority = modifyScore(
	({ priority }) =>
		({
			H: 10,
			M: 5,
			L: 2,
		}[priority] || 0),
);

const fromDue = modifyScore(({ due }) =>
	Math.pow(10, (new Date().getTime() - new Date(due).getTime()) / 60480000),
);

const addScoreToTask = R.pipe(
	R.assoc("score", 0),
	fromPriority,
	fromDue,
);

export default addScoreToTask;
