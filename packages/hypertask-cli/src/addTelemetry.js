import os from "os";
import { setObj } from "@hypercortex/wrapper";

const addTelemetry = db =>
	setObj(db, "telemetry", db.local.key.toString("hex"), {
		hostname: os.hostname(),
		lastAccessed: new Date().toISOString(),
	});

export default addTelemetry;
