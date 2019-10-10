import * as React from "react";

export type Config = {
	kind: "git";
	remote: string;
	username: string;
	token: string;
	scoringFunction: string;
} | null;

type SetConfig =
	| ((newConfig: Config) => void)
	| ((up: (oldConfig: Config) => Config) => void);

const CONFIG_KEY = "config";

export default function useConfig(): [Config, SetConfig] {
	const localStorage_ = process.browser ? localStorage : {};

	const [config, setConfig] = React.useState(
		JSON.parse(localStorage_[CONFIG_KEY] || "null")
	);

	React.useEffect(() => {
		console.log("hello", global, window, localStorage_);
		localStorage_[CONFIG_KEY] = JSON.stringify(config);
	}, [config]);

	return [config, setConfig];
}
