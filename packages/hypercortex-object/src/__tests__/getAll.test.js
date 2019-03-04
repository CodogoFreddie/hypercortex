import hyperdb from "hyperdb";
import ram from "random-access-memory";
import * as R from "ramda";

import wrapperGenerator from "../index";

describe("scalars", () => {
	let db;
	let objectTypeGenerator;
	let getAll;

	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: { scalars: ["description", "index"] },
		calculateScore: async x => {
			const i = await x.indexGet();
			return i;
		},
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json", reduce: a => a });

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

	it("will return a list that is sorted by the object's score", async () => {
		for (const i in R.times(R.identity, 5)) {
			const obj = objectTypeGenerator(`id_${i}`);

			await obj.indexSet(i);
			await obj.descriptionSet(`description ${i}`);
		}

		const objects = await getAll();

		const descriptions = await Promise.all(
			[...objects].map(obj => obj.descriptionGet()),
		);

		expect(descriptions).toEqual([
			"description 4",
			"description 3",
			"description 2",
			"description 1",
			"description 0",
		]);
	});
});
