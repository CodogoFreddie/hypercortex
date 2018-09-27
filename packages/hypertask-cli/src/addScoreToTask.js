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
			L: -3,
		}[priority] || 0),
);

const fromDue = modifyScore(
	({ due }) =>
		10 *
		Math.pow(
			Math.E,
			(new Date().getTime() - new Date(due).getTime()) / 864000000,
		),
);

const fromTimelyOverDue = modifyScore(
	({ due, tags, score }) =>
		new Date().getTime() - new Date(due).getTime() > 0 &&
		tags.includes("timely")
			? 10
			: 0,
);

const fromUrgent = modifyScore(
	({ tags, score }) => (tags.includes("urgent") ? score : 0),
);

const fromAge = modifyScore(
	({ modifiedAt }) =>
		(new Date().getTime() - new Date(modifiedAt).getTime()) / 864000000,
);

const addScoreToTask = R.pipe(
	R.assoc("score", 0),
	fromAge,
	fromPriority,
	fromDue,
	fromTimelyOverDue,
	fromUrgent,
);

export default addScoreToTask;
