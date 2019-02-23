import * as R from "ramda";

const parsePlus = R.pipe(
	R.replace("+", ""),
	R.objOf("plus"),
);

const parseMinus = R.pipe(
	R.replace("-", ""),
	R.objOf("minus"),
);

const parseProp = R.pipe(
	R.replace("_", " "),
	R.split(":"),
	R.over(R.lensIndex(1), x => x || null),
	x => [x],
	R.fromPairs,
	R.objOf("prop"),
);

const parseSaucies = R.pipe(
	R.map(
		R.cond([
			[R.test(/^\+/), parsePlus],
			[R.test(/^-/), parseMinus],
			[R.test(/[a-z]:[\w- ]+$/), parseProp],
			[R.T, R.objOf("plain")],
		]),
	),
	R.partition(R.prop("plain")),
	R.over(
		R.lensIndex(0),
		R.pipe(
			R.map(R.values),
			R.flatten,
			R.join(" "),
			R.objOf("description"),
			R.objOf("prop"),
		),
	),
	R.apply(R.append),
);

const partitionCommandsAndArgs = commands =>
	R.pipe(
		R.slice(2, Infinity),
		R.splitWhen(x => commands[x]),
		R.when(R.pathEq([1, "length"], 0), R.append([])),
		([filter, [command, ...modifications]]) => ({
			filter,
			command,
			modifications,
		}),
		R.evolve({
			filter: parseSaucies,
			modifications: parseSaucies,
		}),
	);

export default partitionCommandsAndArgs;
