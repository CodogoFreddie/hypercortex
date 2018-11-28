import * as R from "ramda";
import createHypercortexObject from "@hypercortex/hypercortex-object";

const modifyScore = fn => task => ({
	...task,
	score: task.score + (fn(task) || 0),
});

const fromDue = modifyScore(
	({ due }) =>
		10 *
		Math.pow(
			Math.E,
			(new Date().getTime() - new Date(due).getTime()) / 864000000,
		),
);

const fromTimelyOverDue = modifyScore(({ due, tags, score }) =>
	new Date().getTime() - new Date(due).getTime() > 0 &&
	tags.includes("timely")
		? 10
		: 0,
);

const fromUrgent = modifyScore(({ tags, score }) =>
	tags.includes("urgent") ? score : 0,
);

const fromAge = modifyScore(({ modifiedAt, due }) =>
	!!due
		? 0
		: (new Date().getTime() - new Date(modifiedAt).getTime()) / 864000000,
);

const createTaskObject = createHypercortexObject({
	type: "task",
	calculateScore: async t => {
		const [due, modifiedAt, tags] = await Promise.all([
			t.dueGet(),
			t.modifiedAtGet(),
			t.tagsGet(),
		]);

		return R.pipe(
			R.assoc("score", 0),
			fromDue,
			fromTimelyOverDue,
			fromUrgent,
			fromAge,
			R.prop("score"),
		)({
			due,
			modifiedAt,
			tags,
		});
	},
	properties: {
		scalars: [
			"description",
			"due",
			"wait",
			"recur",
			"done",
			"createdAt",
			"modifiedAt",
		],
		collections: ["tags"],
	},
});

export default createTaskObject;
