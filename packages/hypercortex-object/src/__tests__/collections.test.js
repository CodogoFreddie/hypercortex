import hyperdb from "hyperdb";
import ram from "random-access-memory";

import wrapperGenerator from "../index";

describe("collections", () => {
	let db;
	let objectTypeGenerator;
	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: {
			collections: [
				"unsorted",
				{
					name: "sorted",
					sortBy: x =>
						x
							.split("")
							.reverse()
							.join(""),
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

	it("can add and retrieve a value to a collection", async () => {
		const obj = objectTypeGenerator("id");

		await obj.unsortedAdd("payload");
		const storedValues = await obj.unsortedGet();
		expect(storedValues).toEqual(["payload"]);
	});

	it("can add and remove a value to a collection", async () => {
		const obj = objectTypeGenerator("id");

		await obj.unsortedAdd("payload");
		await obj.unsortedRemove("payload");
		const storedValues = await obj.unsortedGet();
		expect(storedValues).toEqual([]);
	});

	it("can add and retrieve an object to a collection", async () => {
		const obj = objectTypeGenerator("id");

		await obj.unsortedAdd({
			payload: 123,
			bool: true,
			name: "little chicken",
		});

		await obj.unsortedRemove({
			payload: 123,
			bool: true,
			name: "little chicken",
		});

		const storedValues = await obj.unsortedGet();
		expect(storedValues).toEqual([]);
	});

	it("can add and retrieve many values to a collection", async () => {
		const obj = objectTypeGenerator("id");

		await obj.unsortedAdd("ham");
		await obj.unsortedAdd("jam");
		await obj.unsortedAdd("spam");
		await obj.unsortedAdd("lamb");

		const storedValues = await obj.unsortedGet();
		expect(storedValues).toHaveLength(4);
		expect(storedValues).toContain("ham");
		expect(storedValues).toContain("jam");
		expect(storedValues).toContain("spam");
		expect(storedValues).toContain("lamb");
	});

	it("can retrieve many values sorted", async () => {
		const obj = objectTypeGenerator("id");

		await obj.sortedAdd("ham");
		await obj.sortedAdd("jam");
		await obj.sortedAdd("spam");
		await obj.sortedAdd("lamb");

		const storedValues = await obj.sortedGet();

		expect(storedValues).toEqual(["lamb", "ham", "jam", "spam"]);
	});
});
