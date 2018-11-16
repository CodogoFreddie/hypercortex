import hyperdb from "hyperdb";
import ram from "random-access-memory";

import wrapperGenerator from "../index";

describe("scalars", () => {
	let db;
	let objectTypeGenerator;
	const testObjectSpecification = wrapperGenerator({
		type: "testObject",
		properties: { scalars: ["foo", "bar"] },
	});

	beforeEach(done => {
		db = hyperdb(ram, { valueEncoding: "json" });

		db.on("ready", () => {
			objectTypeGenerator = testObjectSpecification(db).testObject;

			done();
		});
	});

	it("can create and retrieve a property", async () => {
		const obj = objectTypeGenerator("id");
		await obj.fooSet("payload");
		const storedValue = await obj.fooGet();
		expect(storedValue).toBe("payload");
	});

	it("can create and retrieve multipul distinct properties", async () => {
		const obj = objectTypeGenerator("id");

		await obj.fooSet("payload1");
		await obj.barSet("payload2");

		const storedValue1 = await obj.fooGet();
		const storedValue2 = await obj.barGet();

		expect(storedValue1).toBe("payload1");
		expect(storedValue2).toBe("payload2");
	});

	it("can create and retrieve properties on distinct objects", async () => {
		const obj1 = objectTypeGenerator("id1");
		const obj2 = objectTypeGenerator("id2");

		await obj1.fooSet("payload1");
		await obj2.fooSet("payload2");

		const storedValue1 = await obj1.fooGet();
		const storedValue2 = await obj2.fooGet();

		expect(storedValue1).toBe("payload1");
		expect(storedValue2).toBe("payload2");
	});
});
