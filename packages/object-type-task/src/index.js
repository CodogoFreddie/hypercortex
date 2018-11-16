import createHypercortexObject from "@hypercortex/hypercortex-object";

const createTaskObject = createHypercortexObject({
	type: "task",
	properties: {
		scalars: ["description", "due", "wait", "recur", "done"],
		collections: ["tags"],
	},
});

export default createTaskObject;
