import * as R from "ramda";
import hyperdb from "hyperdb";
import ram from "random-access-memory";

import wrapperGenerator from "../index";

describe("delete", () => {
	let db;
	let objectTypeGenerator;
	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: { scalars: ["foo", "bar"] },
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json", reduce: a => a });

		db.on("ready", () => {
			objectTypeGenerator = testObjectSpecification(db).testObject;

			done();
		});
	});

	it("will delete all the values of a hypercortex object", async () => {
		const obj = objectTypeGenerator("id");

		await obj.fooSet("payload1");
		await obj.barSet("payload2");

		const keysBefore = await new Promise(done =>
			db.list("/data", { recursive: true }, (_, x) => done(x)),
		);

		expect(keysBefore).toHaveLength(2);

		const response = await obj.delete();

		expect(response).toEqual([
			"data/testObject/id/foo",
			"data/testObject/id/bar",
		]);

		const keysAfter = await new Promise(done =>
			db.list("/data", { recursive: true }, (_, x) => done(x)),
		);

		expect(keysAfter).toHaveLength(0);
	});
});
