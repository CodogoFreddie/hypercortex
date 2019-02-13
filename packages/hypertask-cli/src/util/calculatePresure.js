import * as R from "ramda";

const calculatePresure = R.pipe(
	R.reduce(
		(accs, {due, score} ) => R.over(
			R.lensProp( 
				due > new Date().toISOString() ? "overdueScore" : "underdueScore"
			),
			R.add(Math.sqrt(score)),
		)(accs),
		{overdueScore: 0, underdueScore: 0}
	),

	({ overdueScore, underdueScore }) => ( overdueScore) + (0.5 * underdueScore),

	x => Math.round(x),
);

export default calculatePresure
