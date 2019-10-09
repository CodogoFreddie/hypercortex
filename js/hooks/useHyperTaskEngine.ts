import * as React from "react";

export type Prop = {
	Description: string;
};

export type Mutation = {
	SetProp: Prop;
};

export type Command = {
	Create: Mutation[];
};

export type Task = {
	id: string;
	description: string;
};

export type HyperTaskRunFunction = (
	cmd_raw: Command,
	updater_fn: (task: Task) => void,
	input_iter_raw: Iterator<Task>
) => void;

type HyperTaskRunFunctionRef = {
	current: HyperTaskRunFunction | null | undefined;
};

export default function useHyperTaskEngine():
	| undefined
	| null
	| HyperTaskRunFunction {
	const [loading, setLoading] = React.useState(true);
	const hypertaskRunFunction: HyperTaskRunFunctionRef = React.useRef();

	React.useEffect(() => {
		try {
			import("@freddieridell/hypertask_npm_package")
				.then(({ run }) => {
					hypertaskRunFunction.current = run;
					setLoading(false);
				})
				.catch(ce => console.log({ ce }));
		} catch (ie) {
			console.log({ ie });
		}
	}, []);

	return loading ? null : hypertaskRunFunction.current;
}
