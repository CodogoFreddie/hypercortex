import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";
import { setObj, getObj, getObjs } from "@hypercortex/wrapper";
import { toDate, addDays, addWeeks, addMonths, addYears } from "date-fns/fp";

import modifyTasks from "./modifyTasks";

const markAsDone = async (db, modifications, filter) => {
	for await (const id of getObjs(db, "task")) {
		if (filter.some(prefix => id.startsWith(prefix))) {
			const { recur, due } = await getObj(db, "task", id);
			if (recur && due) {
				const incrementer = R.pipe(
					toDate,
					{
						d: addDays,
						w: addWeeks,
						m: addMonths,
						y: addYears,
					}[recur.period](recur.n),
				);

				const newDue = incrementer(due);

				await setObj(db, "task", id, {
					due: newDue,
				});
			} else {
				await setObj(db, "task", id, {
					done: new Date().toISOString(),
				});
			}
		}
	}
};

export default markAsDone;
