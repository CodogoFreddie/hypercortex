import hyperdb from "hyperdb";
import ram from "random-access-memory";
import * as R from "ramda";

import wrapperGenerator from "../index";

describe("toObject", () => {
	let db;
	let objectTypeGenerator;
	let getAll;

	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: {
			scalars: ["description", "due"],
			collections: ["tags", "buttons"],
		},
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json" });

		db.on("ready", () => {
			const { testObject, testObjectAll } = testObjectSpecification(db);
			objectTypeGenerator = testObject;

			done();
		});
	});

	it("creates an object based on inputs", async () => {
		const obj = objectTypeGenerator("id");

		await obj.descriptionSet("this is the description");
		await obj.dueSet(2018);

		await obj.tagsAdd("Tag1");
		await obj.tagsAdd("Tag2");

		await obj.buttonsAdd("button1");
		await obj.buttonsAdd("button2");

		const riefiedObject = await obj.toJsObject();

		expect(riefiedObject).toMatchInlineSnapshot(`
Object {
  "buttons": Array [
    "button1",
    "button2",
  ],
  "description": "this is the description",
  "due": 2018,
  "tags": Array [
    "Tag2",
    "Tag1",
  ],
}
`);
	});
});
