import * as R from "ramda";
import createHypercortexObject from "@hypercortex/hypercortex-object";

const addDueToScore = due => score =>
	due
		? score +
		  10 *
				Math.pow(
					Math.E,
					(new Date().getTime() - new Date(due).getTime()) /
						864000000,
				)
		: score;

const createTaskObject = createHypercortexObject({
	type: "task",
	calculateScore: async t => {
		const due = await t.dueGet();
		return R.pipe(addDueToScore(due))(0);
	},
	properties: {
		scalars: ["description", "due", "wait", "recur", "done"],
		collections: ["tags"],
	},
});

export default createTaskObject;
