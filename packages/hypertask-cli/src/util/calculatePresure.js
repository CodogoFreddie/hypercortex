import * as R from "ramda";

const calculatePresure = async taskAll => {
	const tasks = await taskAll();

	const importantProps = await Promise.all(
		tasks.map(task =>
			Promise.all([task.scoreGet(), task.dueGet(), task.doneGet()]).then(
				([score, due, done]) => ({ score, due, done }),
			),
		),
	);

	return R.pipe(
		R.reject(R.prop("done")),
		R.reduce(
			(accs, { due, score }) =>
				R.over(
					R.lensProp(
						due > new Date().toISOString()
							? "overdueScore"
							: "underdueScore",
					),
					R.add(Math.sqrt(score)),
				)(accs),
			{ overdueScore: 0, underdueScore: 0 },
		),

		({ overdueScore, underdueScore }) => overdueScore + 0.5 * underdueScore,

		x => Math.round(x),
	)(importantProps);
};

export default calculatePresure;
