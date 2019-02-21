import * as R from "ramda";
import winSize from "window-size";
import { formatDistanceWithOptions } from "date-fns/fp";

const wrap = (width, prefix = "") =>
	R.pipe(
		R.defaultTo(""),
		R.split(" "),
		R.reduce(
			([head, ...tail], word) => {
				if ((head + " " + word).length > width - prefix.length) {
					return [word, head, ...tail];
				} else {
					return [head + " " + word, ...tail];
				}
			},
			[""],
		),
		R.map(x => prefix + x.trim()),
		R.reverse,
		R.join("\n"),
	);

const renderPin = async pin => {
	const pinObj = await pin.toJsObject();

	const { width } = winSize.get();
	const lines = [
		"",
		`\u001b[0;36m${pinObj.id}\u001b[0m`,

		`added: ${formatDistanceWithOptions(
			{ addSuffix: true },
			new Date(),
			pinObj.createdAt,
		)}`,

		`\u001b[0;34m${pinObj.url}\u001b[0m`,

		pinObj.title && wrap(width, "    ## ")(pinObj.title),

		pinObj.description &&
			`\u001b[0;33m${wrap(width, "    > ")(pinObj.description)}\u001b[0m`,
	].filter(Boolean);

	return "\n" + lines.join("\n");
};

export default renderPin;
