import * as R from "ramda";

let stamps = [];
let stack = [];

const getStamps = () => getStamps;

const profilePromise = async (label, promise) => {
	const startTime = new Date();

	stack.push(label);
	const re = await promise;
	stack.pop();

	const endTime = new Date();

	stamps.push([[...stack, label], endTime.getTime() - startTime.getTime()]);

	return re;
};

export default profilePromise;

export const printStamps = () => {
	if (!process.env.LOG_TIME) {
		return;
	}

	const stampsObj = stamps.reduce(
		(acc, [path, value]) =>
			R.assocPath([...path, "__timestamp"], value, acc),
		{},
	);

	(function recursive(stamps, i = 0) {
		let printIndented = (value) =>
			console.error(new Array(i).fill(null).join(" ") + value);

		R.toPairs(stamps).forEach(([key, value]) => {
			if (key === "__timestamp"){
				return;
			}
			printIndented(key + ": " + value.__timestamp);
			recursive(value, i + 4);
		});
	})(stampsObj);
};
