import createHypercortexObject from "@hypercortex/hypercortex-object";

const createTaskObject = createHypercortexObject({
	type: "task",
	calculateScore: async t => {
		const due = await t.dueGet();
		return (
			10 *
			Math.pow(
				Math.E,
				(new Date().getTime() - new Date(due).getTime()) / 864000000,
			)
		);
	},
	properties: {
		scalars: ["description", "due", "wait", "recur", "done"],
		collections: ["tags"],
	},
});

export default createTaskObject;
