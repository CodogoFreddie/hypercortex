import * as R from "ramda";
import { getObjs, setObj } from "@hypercortex/wrapper";

const markTasksAs = async (db, filter, prop, value) => {
	console.log(filter);
	for await (const id of getObjs(db, "task")) {
		if (filter.some(filter => id.startsWith(filter))) {
			await setObj(db, "type", id, {
				[prop]: value,
			});
		}
	}
};

export default markTasksAs;
