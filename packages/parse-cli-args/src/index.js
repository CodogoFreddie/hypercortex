import * as R from "ramda";
import {
	parse,
	addHours,
	addDays,
	addWeeks,
	addMonths,
	addYears,
} from "date-fns/fp";

const parseTaged = (sign, name) =>
	R.pipe(
		R.replace(sign, ""),
		R.objOf(name),
	);

const parsePlusTag = parseTaged("+", "plusTag");
const parseMinusTag = parseTaged("-", "minusTag");
const parseString = parseTaged("", "string");

const parseTimeDeltaValue = R.pipe(
	x => /^(\d+)([hdwmy])$/.exec(x),
	([_, n, period]) => ({
		n,
		period,
	}),
	R.evolve({
		n: n => parseInt(n, 10),
	}),
	R.applySpec({
		deltaObject: R.identity,
		fromNow: ({ n, period }) =>
			R.pipe(
				{
					h: addHours,
					d: addDays,
					w: addWeeks,
					m: addMonths,
					y: addYears,
				}[period](n),
				x => x.toISOString(),
			)(new Date()),
	}),
);

const parsePropValue = R.pipe(
	R.cond([
		[R.test(/^\d{2}-\d{2}$/), parse(new Date(), "MM-dd")],
		[R.test(/^\d{4}$/), parse(new Date(), "yyyy")],
		[R.test(/^\d{4}-\d{2}$/), parse(new Date(), "yyyy-MM")],
		[R.test(/^\d{4}-\d{2}-\d{2}$/), parse(new Date(), "yyyy-MM-dd")],

		[R.test(/^\d+[hdwmy]$/), parseTimeDeltaValue],
		[R.test(/^now$/), () => new Date()],

		[R.T, R.identity],
	]),

	R.when(
		x => x instanceof Date,
		x => x.toISOString(),
	),
);

const parseProp = R.pipe(
	R.split(":"),
	R.over(R.lensIndex(1), parsePropValue),
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

export default commands =>
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
