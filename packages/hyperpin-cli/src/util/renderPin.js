import * as R from "ramda";
import winSize from "window-size";

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
		pinObj.id,
		pinObj.score,
		pinObj.url,
		"",
		wrap(width, "    ## ")(pinObj.title),
		wrap(width, "    > ")(pinObj.description),
	];

	return lines.join("\n");
};

export default renderPin;
