import hyperdb from "hyperdb";
import ram from "random-access-memory";

import { readyGate, getObjs, createObj, setObj, getObj } from "../../wrapper";

const db = hyperdb(ram, { valueEncoding: "json" });

const setter = setObj(db, "task");
const getter = getObj(db, "task");
const creater = createObj(db, "task");

const foo = async () => {
	await readyGate(db);

	for (let i = 0; i < 5; i++) {
		await creater(i);

		await setter(i)({
			title: "THE_TITLE",
			description: "THE_DESCRIPTION",
			due: new Date().toISOString(),
		});
	}

	for await (const id of getObjs(db, "task")) {
		console.log(id);
		const obj = await getter(id);
		console.log(obj);
	}
};

foo();
