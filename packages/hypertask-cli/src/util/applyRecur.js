import { addHours, addDays, addMonths, addWeeks, addYears } from "date-fns/fp";

const applyRecur = ({ n, period }) =>
	({
		h: addHours,
		d: addDays,
		m: addMonths,
		w: addWeeks,
		y: addYears,
	}[period](n));

export default applyRecur;
