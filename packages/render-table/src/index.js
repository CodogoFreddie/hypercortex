import * as R from "ramda";
import winSize from "window-size";

const renderTable = R.curry((columns, data) => {
	const dataWeHaveSpaceToRender = data.slice(0, winSize.get().height - 4);

	const columnWidths = dataWeHaveSpaceToRender.reduce(
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

	const header =
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
		"\u001b[0m";

	const lines = [header];

	let i = 0;
	for (const datum of dataWeHaveSpaceToRender) {
		const line = [];

		for (const col of columns) {
			line.push(("" + (datum[col] || "")).padEnd(columnWidths[col]));
		}

		const { textColor } = data[i++];

		if (textColor) {
			lines.push(`\u001b[1;3${textColor}m${line.join(" ")}\u001b[0m`);
		} else {
			lines.push(line.join(" "));
		}
	}

	return lines.join("\n");
});

export default renderTable;
