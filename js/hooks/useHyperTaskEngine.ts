import * as React from "react";
import * as git from "isomorphic-git";
import * as LightningFS from "@isomorphic-git/lightning-fs";
import * as R from "ramda";

import { Config } from "./useConfig";

export type Prop = {
	Description: string;
};

export type Tag = {
	sign: "Plus" | "Minus";
	name: string;
};

export type Mutation = {
	SetProp: Prop;
	SetTag: Tag;
};

export type Query = {};

export type Command = {
	Read: Query[];
	Create: Mutation[];
};

export type Task = {
	id: string;
	description: string;
};

export type HyperTaskRunFunction = (cmd_raw: Command) => Task[];

type HyperTaskRunFunctionRef = {
	current: HyperTaskRunFunction | null | undefined;
};

const GIT_DIR = "/hypertask";
const dir = GIT_DIR;

function loadGit(config: Config) {
	return new Promise(done => {
		window.fs = new LightningFS("fs");
		git.plugins.set("fs", window.fs);
		window.pfs = window.fs.promises;
		done();
	})
		.then(() =>
			git
				.log({ dir })
				.then(() => git.fetch)
				.catch(() => git.clone)
				.then(
					fn => (
						console.log(fn),
						fn({
							corsProxy: "https://cors.isomorphic-git.org",
							depth: 1,
							dir: GIT_DIR,
							ref: "master",
							singleBranch: true,
							token: config.token,
							url: config.remote,
						})
					)
				)
				.then(() =>
					git
						.checkout({ dir: GIT_DIR, ref: "master" })
						.then(console.log)
				)
		)
		.catch(console.error);
}

function loadEngine(): Promise<HyperTaskRunFunction> {
	return import("@freddieridell/hypertask_npm_package")
		.then(({ run }) => run)
		.catch(ce => console.error({ ce }));
}

type TaskCache = Record<String, Task | undefined>;

function getAllTasks(config: Config): Promise<TaskCache> {
	return Promise.resolve()
		.then(() => pfs.readdir([dir, config.taskDir].join("/")))
		.then(ids =>
			Promise.all(
				ids.map(id =>
					pfs
						.readFile(
							[dir, config.taskDir, id]
								.join("/")
								.replace(/\/+/g, "/"),
							{ encoding: "utf8" }
						)
						.then(JSON.parse)
				)
			)
		)
		.then(
			R.pipe(
				R.map(task => [task.id, task]),
				R.fromPairs
			)
		);
}

export default function useHyperTaskEngine(
	config: Config
): undefined | null | HyperTaskRunFunction {
	const [loading, setLoading] = React.useState(true);
	const [tasksCache, setTasksCache] = React.useState({});
	const hypertaskRunFunction: HyperTaskRunFunctionRef = React.useRef();

	const run = React.useCallback(
		(cmd: Command): Task[] =>
			hypertaskRunFunction.current(
				cmd,
				task => console.log("update", task),
				Object.values(tasksCache),
				config.scoringFunction
			),

		[tasksCache]
	);

	React.useEffect(() => {
		Promise.all([loadEngine(), loadGit(config)])
			.then(([run]) => {
				hypertaskRunFunction.current = run;
			})
			.then(() => getAllTasks(config))
			.then(R.tap(console.log))
			.then(setTasksCache)
			.then(() => {
				setLoading(false);
			})
			.catch(console.error);
	}, []);

	return loading ? null : run;
}
