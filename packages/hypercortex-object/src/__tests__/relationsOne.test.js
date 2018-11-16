import hyperdb from "hyperdb";
import ram from "random-access-memory";

import wrapperGenerator from "../index";

describe("scalars", () => {
	let db;
	let objectTypeGenerator;

	const subObjectSpecification = wrapperGenerator({
		type: "subObject",
		properties: {
			scalars: ["description"],
		},
	});

	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		relations: {
			one: [
				{
					name: "subType",
					type: "subObject",
					resolver: subObjectSpecification,
				},
			],
		},
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json" });

		db.on("ready", () => {
			objectTypeGenerator = testObjectSpecification(db).testObject;
			done();
		});
	});

	it("can create and get a newSubtype", async () => {
		const obj = objectTypeGenerator("root");

		await obj.subTypeCreate();

		const createdSubType = await obj.subTypeGet();

		expect(createdSubType.typeGet()).toBe("subObject");
	});
});
