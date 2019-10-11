import * as React from "react";
import Router from "next/router";
import * as R from "ramda";

import useConfig from "../hooks/useConfig";

import Shell from "../components/Shell";
import TextInput from "../components/TextInput";
import Button from "../components/Button";

const Config = () => {
	const [config, setConfig] = useConfig();

	React.useEffect(() => {
		if (!config) {
			setConfig({
				kind: "git",
				remote: "",
				username: "",
				token: "",
				scoringFunction: "due :",
				taskDir: "",
			});
		}
	}, [config]);

	if (!config) {
		return null;
	}

	return (
		<Shell>
			<TextInput
				kind="text"
				label="Remote"
				value={config.remote}
				onChange={remote => setConfig(R.assoc("remote", remote))}
			/>
			<TextInput
				kind="text"
				label="Username"
				value={config.username}
				onChange={username => setConfig(R.assoc("username", username))}
			/>
			<TextInput
				kind="password"
				label="Token"
				value={config.token}
				onChange={token => setConfig(R.assoc("token", token))}
			/>
			<TextInput
				kind="text"
				label="Task Directory"
				value={config.taskDir}
				onChange={taskDir => setConfig(R.assoc("taskDir", taskDir))}
			/>
			<TextInput
				kind="text"
				label="Scoring Function"
				value={config.scoringFunction}
				onChange={scoringFunction =>
					setConfig(R.assoc("scoringFunction", scoringFunction))
				}
			/>

			<Button onClick={() => Router.pop()}>Back</Button>
		</Shell>
	);
};

export default Config;
