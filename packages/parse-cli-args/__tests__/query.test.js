import { advanceBy, advanceTo, clear } from "jest-date-mock";

import createParseCliArgs from "../src";

describe("query", () => {
	let parseCliArgs;

	beforeEach(() => {
		advanceTo(new Date(2001, 1, 3, 4, 5, 6)); // reset to date time.

		parseCliArgs = createParseCliArgs(["add", "modify", "delete"]);
	});

	afterEach(() => {
		clear();
	});

	describe("plus tags", () => {
		it("parses one", () => {
			const response = parseCliArgs(["+foo"]);

			expect(response).toMatchObject({
				query: [{ plusTag: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["+foo", "+bar", "+baz"]);

			expect(response).toMatchObject({
				query: [
					{ plusTag: "foo" },
					{ plusTag: "bar" },
					{ plusTag: "baz" },
				],
			});
		});
	});

	describe("minus tags", () => {
		it("parses one", () => {
			const response = parseCliArgs(["-foo"]);

			expect(response).toMatchObject({
				query: [{ minusTag: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["-foo", "-bar", "-baz"]);

			expect(response).toMatchObject({
				query: [
					{ minusTag: "foo" },
					{ minusTag: "bar" },
					{ minusTag: "baz" },
				],
			});
		});
	});

	describe("strings", () => {
		it("parses one", () => {
			const response = parseCliArgs(["foo"]);

			expect(response).toMatchObject({
				query: [{ string: "foo" }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["foo", "bar", "baz"]);

			expect(response).toMatchObject({
				query: [
					{ string: "foo" },
					{ string: "bar" },
					{ string: "baz" },
				],
			});
		});
	});

	describe("props", () => {
		it("parses one", () => {
			const response = parseCliArgs(["foo:bar"]);

			expect(response).toMatchObject({
				query: [{ prop: { foo: "bar" } }],
			});
		});

		it("parses many", () => {
			const response = parseCliArgs(["foo:bar", "baz:qux", "a:b"]);

			expect(response).toMatchObject({
				query: [
					{ prop: { foo: "bar" } },
					{ prop: { baz: "qux" } },
					{ prop: { a: "b" } },
				],
			});
		});

		describe("parsing absolute date shortcuts", () => {
			[
				["2019", "2019-01-01T00:00:00.000Z"],
				["2019-03", "2019-03-01T00:00:00.000Z"],
				["2019-03-05", "2019-03-05T00:00:00.000Z"],
				["03-05", "2019-03-05T00:00:00.000Z"],
				["now", "2001-02-03T04:05:06.000Z"],
			].forEach(([input, output]) => {
				test(`due:${input} parses to '${output}'`, () => {
					const response = parseCliArgs([`due:${input}`]);

					expect(response).toMatchObject({
						query: [{ prop: { due: output } }],
					});
				});
			});
		});

		describe("parsing relative date shortcuts", () => {
			[
				["1d", { n: 1, period: "d" }, "2001-02-04T04:05:06.000Z"],
				["3d", { n: 3, period: "d" }, "2001-02-06T04:05:06.000Z"],
				["20d", { n: 20, period: "d" }, "2001-02-23T04:05:06.000Z"],
				["1w", { n: 1, period: "w" }, "2001-02-10T04:05:06.000Z"],
				["3w", { n: 3, period: "w" }, "2001-02-24T04:05:06.000Z"],
				["1m", { n: 1, period: "m" }, "2001-03-03T04:05:06.000Z"],
				["3m", { n: 3, period: "m" }, "2001-05-03T03:05:06.000Z"],
				["1y", { n: 1, period: "y" }, "2002-02-03T04:05:06.000Z"],
			].forEach(([input, objectOutput, dateOutput]) => {
				test(`due:${input} parses to '${JSON.stringify(
					objectOutput,
				)}' & '${dateOutput}`, () => {
					const response = parseCliArgs([`due:${input}`]);

					expect(response).toMatchObject({
						query: [
							{
								prop: {
									due: {
										deltaObject: objectOutput,
										fromNow: dateOutput,
									},
								},
							},
						],
					});
				});
			});
		});
	});

	it("parses a complex array of arguments", () => {
		const response = parseCliArgs([
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
			query: [
				{ prop: { due: "2001-02-03T04:05:06.000Z" } },
				{ prop: { recur: { deltaObject: { n: 2, period: "d" } } } },
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
