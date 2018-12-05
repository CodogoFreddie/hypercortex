import * as R from "ramda";
import chrono from "chrono-node";
import {
	isAfter,
	addHours,
	addDays,
	addMonths,
	addWeeks,
	addYears,
	endOfDay,
	endOfMonth,
	endOfWeek,
	endOfYear,
	isSameHour,
	isSameDay,
	isSameMonth,
	isSameWeek,
	isSameYear,
	startOfDay,
	startOfMonth,
	startOfWeek,
	startOfYear,
	toDate,
	getYear,
} from "date-fns";

const extractNumber = (adder, ref) =>
	R.pipe(
		R.match(/\d+/),
		R.head,
		x => parseInt(x, 10),
		x => adder(ref, x),
	);

const parseMonthDay = ref =>
	R.pipe(
		R.replace(/^/, getYear(ref)),
		toDate,
	);

const nextFree = (discriminator, incrementer, ref) => async taskAll => {
	const tasks = await taskAll();
	const allDues = await Promise.all(tasks.map(t => t.dueGet()));
	const sortedDues = R.pipe(
		R.filter(Boolean),
		R.filter(d => d > ref.toISOString()),
		R.sortBy(R.identity),
		R.map(toDate),
		R.prepend(ref),
	)(allDues);

	const reduced = R.reduce(
		(acc, val) => {
			if (discriminator(acc, val)) {
				return incrementer(acc, 1);
			}
			if (isAfter(/*bigger*/ acc, /*smaller*/ val)) {
				return acc;
			}
			return R.reduced(acc);
		},
		ref,
		sortedDues,
	);

	return reduced.toISOString();
};

export const parseFrom = ref =>
	R.cond([
		[R.test(/^\d+h$/), extractNumber(addHours, ref)],
		[R.test(/^\d+d$/), extractNumber(addDays, ref)],
		[R.test(/^\d+w$/), extractNumber(addWeeks, ref)],
		[R.test(/^\d+m$/), extractNumber(addMonths, ref)],
		[R.test(/^\d+y$/), extractNumber(addYears, ref)],
		[R.test(/^eod$/), () => endOfDay(ref)],
		[R.test(/^eom$/), () => endOfMonth(ref)],
		[R.test(/^eow$/), () => endOfWeek(ref)],
		[R.test(/^eoy$/), () => endOfYear(ref)],
		[R.test(/^sod$/), () => startOfDay(ref)],
		[R.test(/^som$/), () => startOfMonth(ref)],
		[R.test(/^sow$/), () => startOfWeek(ref)],
		[R.test(/^soy$/), () => startOfYear(ref)],
		[R.test(/^now$/), () => ref],
		[R.test(/^free.h$/), () => nextFree(isSameHour, addHours, ref)],
		[R.test(/^free.d$/), () => nextFree(isSameDay, addDays, ref)],
		[R.test(/^free.w$/), () => nextFree(isSameWeek, addWeeks, ref)],
		[R.test(/^free.m$/), () => nextFree(isSameMonth, addMonths, ref)],
		[R.test(/^free.y$/), () => nextFree(isSameYear, addYears, ref)],
		[R.test(/^(\d{4}-)(\d{2})?(-\d{2})?$/), toDate],
		[R.test(/^(\d{2})?(-\d{2})?$/), parseMonthDay(ref)],
		[R.T, chrono.parseDate],
	]);

export default parseFrom(new Date());

export const parseRecur = R.pipe(
	R.match(/(\d+)([hdwmy])/),
	([_, n, period]) => ({
		n: parseInt(n, 10),
		period,
	}),
);
