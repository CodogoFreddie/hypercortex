import createParseCliArgs from "../src";

describe("query", () => {
	let parseCliArgs;

	beforeEach(() => {
		parseCliArgs = createParseCliArgs(["add", "modify", "delete"]);
	});

	describe("plus tags", () => {
		it("parses one", () => {
			const response = parseCliArgs(["modify","+foo"]);

			expect(response).toMatchObject({
				mutation: [{ plusTag: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["modify","+foo", "+bar", "+baz"]);

			expect(response).toMatchObject({
				mutation: [
					{ plusTag: "foo" },
					{ plusTag: "bar" },
					{ plusTag: "baz" },
				],
			});
		});
	});

	describe("minus tags", () => {
		it("parses one", () => {
			const response = parseCliArgs(["modify","-foo"]);

			expect(response).toMatchObject({
				mutation: [{ minusTag: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["modify","-foo", "-bar", "-baz"]);

			expect(response).toMatchObject({
				mutation: [
					{ minusTag: "foo" },
					{ minusTag: "bar" },
					{ minusTag: "baz" },
				],
			});
		});
	});

	describe("strings", () => {
		it("parses one", () => {
			const response = parseCliArgs(["modify","foo"]);

			expect(response).toMatchObject({
				mutation: [{ string: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["modify","foo", "bar", "baz"]);

			expect(response).toMatchObject({
				mutation: [
					{ string: "foo" },
					{ string: "bar" },
					{ string: "baz" },
				],
			});
		});
	});

	describe("props", () => {
		it("parses one", () => {
			const response = parseCliArgs(["modify","foo:bar"]);

			expect(response).toMatchObject({
				mutation: [{ prop: { foo: "bar" } }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["modify","foo:bar", "baz:qux", "a:b"]);

			expect(response).toMatchObject({
				mutation: [
					{ prop: { foo: "bar" } },
					{ prop: { baz: "qux" } },
					{ prop: { a: "b" } },
				],
			});
		});
	});

	it("parses a complex array of arguments", () => {
		const response = parseCliArgs(["modify",
			"due:now",
			"recur:2d",
			"+chores",
			"-timely",
			"do",
			"the",
			"dishes",
			"more",
			"often",
		]);

		expect(response).toMatchObject({
			mutation: [
				{ prop: { due: "now" } },
				{ prop: { recur: "2d" } },
				{ plusTag: "chores" },
				{ minusTag: "timely" },
				{ string: "do" },
				{ string: "the" },
				{ string: "dishes" },
				{ string: "more" },
				{ string: "often" },
			],
		});
	});
});
