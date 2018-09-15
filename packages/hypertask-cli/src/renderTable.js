import * as R from "ramda";
import { getObjs, getObj } from "@hypercortex/wrapper";
import { format, toDate } from "date-fns/fp";

import addScoreToTask from "./addScoreToTask";

const formatTask = R.evolve({
	due: R.pipe(
		toDate,
		format("yy-MM-dd"),
	),
	wait: R.pipe(
		toDate,
		format("yy-MM-dd"),
	),
	score: n => n.toPrecision(2),
});

const tableify = R.curry((columns, data) => {
	const columnWidths = data.reduce(
		(widths, task) => ({
			...widths,
			...R.fromPairs(
				R.toPairs(task).map(([key, value]) => [
					key,
					Math.max(("" + value).length, widths[key] || 0),
				]),
			),
		}),
		R.fromPairs(columns.map(key => [key, key.length])),
	);

	const lines = [
		"\u001b[4m" +
			columns
				.map(
					R.pipe(
						col => col.padEnd(columnWidths[col]),
						R.split(""),
						R.over(R.lensIndex(0), R.toUpper),
						R.join(""),
					),
				)
				.join(" ") +
			"\u001b[0m",
	];

	let even = false;
	for (const datum of data) {
		const line = [];
		for (const col of columns) {
			line.push(("" + (datum[col] || "")).padEnd(columnWidths[col]));
		}

		even = !even;
		lines.push(
			even ? "\u001b[1m" + line.join(" ") + "\u001b[0m" : line.join(" "),
		);
	}

	return lines.join("\n");
});

const hyperTaskTableify = tableify([
	"score",
	"key",
	"id",
	"title",
	"due",
	"wait",
	"priority",
]);

const generateUniqPrefixes = ids => {
	const root = {
		value: null,
		children: {},
	};

	const insert = (root, key, value) => {
		let node = root;
		let indexLastChar = null;

		for (const i in key) {
			const char = key[i];
			if (node.children[char]) {
				node = node.children[char];
			} else {
				indexLastChar = i;
				break;
			}
		}

		if (indexLastChar != null) {
			for (const char of key.slice(indexLastChar)) {
				node.children[char] = {
					value: null,
					children: {},
				};
				node = node.children[char];
			}
		}
		node.value = value;
	};

	const flatten = node => {
		const countChildren = ({ children, value }, n = 0) => {
			if (value) {
				return n + 1;
			} else if (children) {
				return R.pipe(
					R.values,
					R.map(countChildren),
					R.sum,
				)(children);
			} else {
				return n;
			}
		};

		const getOnlyChild = node => {
			if (node.value) {
				return node.value;
			}
			if (countChildren(node) === 1) {
				return getOnlyChild(R.values(node.children)[0]);
			}
		};

		if (countChildren(node) === 1) {
			return {
				value: getOnlyChild(node),
				children: {},
			};
		} else {
			return R.evolve({
				children: R.map(flatten),
			})(node);
		}
	};

	ids.forEach(id => insert(root, id, id));

	const flattenedTrie = flatten(root);

	const prefixes = {};

	const walkTrie = ({ children, value }, path = "") => {
		if (value) {
			prefixes[value] = path;
		}

		for (const char of R.keys(children)) {
			walkTrie(children[char], path + char);
		}
	};

	walkTrie(flattenedTrie);

	return prefixes;
};

const renderTable = async db => {
	const tasks = {};
	const getTask = getObj(db, "task");

	for await (const id of getObjs(db, "task")) {
		const rawTask = await getTask(id);

		tasks[id] = R.pipe(
			addScoreToTask,
			formatTask,
		)(rawTask);
	}

	const ids = generateUniqPrefixes(R.keys(tasks));

	for (const id in ids) {
		tasks[id].key = ids[id];
	}

	const tasksSorted = R.pipe(
		R.values,
		R.sort(
			R.descend(
				R.pipe(
					R.prop("score"),
					Number,
				),
			),
		),
	)(tasks);

	const renderedString = hyperTaskTableify(tasksSorted);

	console.log(renderedString);
};

export default renderTable;
