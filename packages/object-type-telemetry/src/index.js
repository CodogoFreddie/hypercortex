import createHypercortexObject from "@hypercortex/hypercortex-object";

const createTelemetryObject = createHypercortexObject({
	type: "telemetry",
	properties: {
		scalars: ["name", "key"],
		collections: ["ips"],
	},
});

export default createTelemetryObject;
