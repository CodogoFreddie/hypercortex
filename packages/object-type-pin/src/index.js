import createHypercortexObject from "@hypercortex/hypercortex-object";

const createTelemetryObject = createHypercortexObject({
	type: "pin",
	calculateScore: async pin => {
		const [createdAt, archived] = await Promise.all([
			pin.createdAtGet(),
			pin.archivedGet(),
		]);

		if (archived) {
			return 0;
		}

		return 10000000000000 - new Date(createdAt).getTime();
	},
	properties: {
		scalars: [
			"createdAt",
			"description",
			"keywords",
			"title",
			"url",
			"tags",
			"archived",
		],
		collections: ["tags"],
	},
});

export default createTelemetryObject;
