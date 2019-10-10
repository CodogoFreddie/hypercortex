import * as React from "react";
import * as git from "isomorphic-git";
import * as LightningFS from "@isomorphic-git/lightning-fs";

import { Config } from "./useConfig";

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

const GIT_DIR = "/hypertask";
const dir = GIT_DIR;

function useGit() {
	const [loaded, setLoaded] = React.useState(false);
	React.useEffect(() => {
		window.fs = new LightningFS("fs");
		git.plugins.set("fs", window.fs);
		// I prefer using the Promisified version honestly
		window.pfs = window.fs.promises;
		console.log("done");
		setLoaded(true);
	}, []);

	return loaded;
}

export default function useHyperTaskEngine(
	config: Config
): undefined | null | HyperTaskRunFunction {
	const [loading, setLoading] = React.useState(true);
	const hypertaskRunFunction: HyperTaskRunFunctionRef = React.useRef();
	const gitLoaded = useGit();

	React.useEffect(() => {
		try {
			import("@freddieridell/hypertask_npm_package")
				.then(({ run }) => {
					hypertaskRunFunction.current = run;
					setLoading(false);
				})
				.catch(ce => console.error({ ce }));
		} catch (ie) {
			console.log({ ie });
		}
	}, []);

	React.useEffect(() => {
		if (gitLoaded) {
			console.log(4);

			git.clone({
				corsProxy: "https://cors.isomorphic-git.org",
				depth: 1,
				dir: GIT_DIR,
				ref: "master",
				singleBranch: true,
				token: config.token,
				url: config.remote,
			})
				.then(() => pfs.readdir(dir))
				.then(files => {
					console.log(files);
				})
				.catch(x => console.error(x));
		}
	}, [gitLoaded]);

	return loading ? null : hypertaskRunFunction.current;
}
