import * as R from "ramda";

const parseTaged = (sign, name) =>
	R.pipe(
		R.replace(sign, ""),
		R.objOf(name),
	);

const parsePlusTag = parseTaged("+", "plusTag");
const parseMinusTag = parseTaged("-", "minusTag");
const parseString = parseTaged("", "string");

const parseProp = R.pipe(
	R.split(":"),
	([key, value]) => ({ [key]: value }),
	R.objOf("prop"),
);

const parseArg = R.cond([
	[R.test(/^\+.+/), parsePlusTag],
	[R.test(/^\-.+/), parseMinusTag],
	[R.test(/^\w+:/), parseProp],
	[R.T, parseString],
]);

const parseArgs = R.map(parseArg);

const parseCliArgs = commands =>
	R.pipe(
		R.splitWhen(arg => commands.includes(arg)),
		([query, [command, ...mutation]]) => ({
			query,
			command,
			mutation,
		}),
		R.evolve({
			query: parseArgs,
			mutation: parseArgs,
		}),
   );
export default parseCliArgs;
