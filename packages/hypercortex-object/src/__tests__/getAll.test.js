import hyperdb from "hyperdb";
import ram from "random-access-memory";

import wrapperGenerator from "../index";

describe("scalars", () => {
	let db;
	let objectTypeGenerator;
	let getAll;

	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: { scalars: ["description"] },
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json" });

		db.on("ready", () => {
			const { testObject, testObjectAll } = testObjectSpecification(db);
			objectTypeGenerator = testObject;
			getAll = testObjectAll;

			done();
		});
	});

	it("will return a list of the object that have been created", async () => {
		const foo = objectTypeGenerator("foo");
		const bar = objectTypeGenerator("bar");

		await foo.descriptionSet("foo's description");
		await bar.descriptionSet("bar's description");

		const objects = await getAll();

		const descriptions = await Promise.all(
			[...objects].map(obj => obj.descriptionGet()),
		);

		expect(descriptions).toContain("foo's description");
		expect(descriptions).toContain("bar's description");
	});
});
