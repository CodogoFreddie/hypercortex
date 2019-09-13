import React from "react";
import styled from "@emotion/styled";
import * as R from "ramda";
import { createShadow, calm } from "@freddieridell/little-bonsai-styles";

function hashString(str) {
	var hash = 0;
	if (str.length == 0) {
		return hash;
	}
	for (var i = 0; i < str.length; i++) {
		var char = str.charCodeAt(i);
		hash = (hash << 5) - hash + char;
		hash = hash & hash; // Convert to 32bit integer
	}
	return Math.abs(hash);
}

const TaskContainer = styled.div(
	calm({
		padding: R.path(["theme", "size", "space", 1]),
	}),
);

const TaskTag = styled.li(
	calm({
		backgroundColor: ({ theme, col }) => theme.color.chromatic[col],
		color: R.path(["theme", "color", "symantic", "text"]),

		height: R.path(["theme", "size", "space", 4]),
		borderRadius: R.path(["theme", "size", "space", 4]),
		fontSize: R.path(["theme", "size", "font", 2]),

		padding: R.path(["theme", "size", "space", 1]),
		paddingLeft: R.path(["theme", "size", "space", 2]),
		paddingRight: R.path(["theme", "size", "space", 2]),

		alignItems: "center",
		cursor: "pointer",
		display: "flex",
		justifyContent: "center",

		span: {
			display: "block",
		},
	}),
);

const Tag = ({ children, onClick }) => {
	const hash = hashString(children);

	const col = ["blue", "green", "orange", "purple", "red", "yellow"][
		hash % 6
	];

	return (
		<TaskContainer>
			<TaskTag col={col} onClick={onClick}>
				<span>{children}</span>
			</TaskTag>
		</TaskContainer>
	);
};

export default Tag;
