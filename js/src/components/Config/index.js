import React from "react";
import styled from "@emotion/styled";
import { calm } from "@freddieridell/little-bonsai-styles";
import * as R from "ramda";

export function useConfig() {
	const [config, onChangeConfig] = React.useState(
		JSON.parse(localStorage.getItem("config") || "{}"),
	);

	React.useEffect(() => {
		localStorage.setItem("config", JSON.stringify(config));
	}, [config]);

	return {
		config,
		onChangeConfig,
		configHasBeenSet: Boolean(config.apiURL),
	};
}

const ModalBackground = styled.div(
	calm({
		backgroundColor: "rgba(0,0,0,0.4)",
		bottom: 0,
		left: 0,
		position: "absolute",
		right: 0,
		top: 0,

		display: "flex",
		alignItems: "center",
		justifyContent: "center",
	}),
);

const ModalContainer = styled.div(
	calm({
		backgroundColor: R.path(["theme", "color", "symantic", "background"]),
		borderRadius: R.path(["theme", "size", "space", 2]),
		display: "flex",
		flexDirection: "column",
		padding: R.path(["theme", "size", "space", 2]),
		overflow: "hidden",
	}),
);

const ModalHeader = styled.h3(
	calm({
		alignSelf: "stretch",
		backgroundColor: "rgba(0,0,0,0.4)",
		display: "block",

		margin: R.pipe(
			R.path(["theme", "size", "space", 2]),
			x => `-${x}`,
		),
		marginBottom: 0,
		padding: R.path(["theme", "size", "space", 2]),
	}),
);

const Config = ({ value, onChange }) => {
	const [bufferedConfig, onChangeBufferedConfig] = React.useState(value);

	return (
		<ModalBackground>
			<ModalContainer>
				<ModalHeader>Hypertask client config</ModalHeader>
				API URL:
				<input
					value={value.apiURL}
					onChange={e =>
						onChangeBufferedConfig(
							R.assoc("apiURL", e.target.value),
						)
					}
				/>
				<button
					onClick={() => {
						onChange(bufferedConfig);
					}}
				>
					save
				</button>
			</ModalContainer>
		</ModalBackground>
	);
};

export default Config;
