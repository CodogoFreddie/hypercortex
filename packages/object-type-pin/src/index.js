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
			"archived",
			"author",
			"canonical",
			"createdAt",
			"description",
			"icon",
			"image",
			"keywords",
			"siteName",
			"subject",
			"tags",
			"title",
			"url",
		],
		collections: ["tags"],
	},
});

export default createTelemetryObject;
