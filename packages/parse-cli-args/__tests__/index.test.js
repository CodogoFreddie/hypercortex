"use strict";

import createParseCliArgs from "../src";

describe("@hypercortex/parse-cli-args", () => {
	let parseCliArgs;

	beforeEach(() => {
		parseCliArgs = createParseCliArgs(["add", "modify", "delete"]);
	});

	describe("command", () => {
		it("recognises commands as commands", () => {
			const response = parseCliArgs(["add"]);

			expect(response).toEqual({
				query: [],
				command: "add",
				mutation: [],
			});
		});

		it("partitions non commands to either side of the command", () => {
			const response = parseCliArgs(["foo:bar", "modify", "+tag"]);

			expect(response).toMatchObject({
				command: "modify",
			});
		});

		it("can start with a command", () => {
			const response = parseCliArgs(["delete", "abc"]);

			expect(response).toMatchObject({
				command: "delete",
			});
		});

		it("can end with a command", () => {
			const response = parseCliArgs(["abc", "modify"]);

			expect(response).toMatchObject({
				command: "modify",
			});
		});

		it("picks the first of many commands", () => {
			const response = parseCliArgs(["modify", "delete", "add"]);

			expect(response).toMatchObject({
				command: "modify",
			});
		});
	});
});
