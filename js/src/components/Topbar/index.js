import React from "react";
import styled from "@emotion/styled";
import { calm } from "@freddieridell/little-bonsai-styles";
import * as R from "ramda";

import Tag from "../Tag";

const TopbarStyled = styled.header(
	calm({
		backgroundColor: R.path([
			"theme",
			"color",
			"symantic",
			"inverted",
			"background",
		]),
		color: R.path(["theme", "color", "symantic", "inverted", "text"]),
		fontSize: R.path(["theme", "size", "font", 3]),
		padding: R.path(["theme", "size", "space", 2]),
		alignSelf: "stretch",

		width: "100%",
	}),
);

const QueryRenderer = styled.ul(
	calm({
		backgroundColor: R.path([
			"theme",
			"color",
			"symantic",
			"inverted",
			"background",
		]),
		padding: R.path(["theme", "size", "space", 1]),
		margin: 0,
		color: R.path(["theme", "color", "symantic", "inverted", "text"]),

		display: "flex",
		alignSelf: "stretch",
		alignItems: "center",
		display: "flex",
		justifyContent: "flex-start",

		span: {
			display: "block",
			padding: R.path(["theme", "size", "space", 1]),
		},

		li: {
			margin: R.path(["theme", "size", "space", 1]),
		},
	}),
);

const Topbar = ({ query, setQuery }) => {
	return (
		<React.Fragment>
			<TopbarStyled>HyperTask</TopbarStyled>

			{query.length > 0 && (
				<QueryRenderer>
					<span>Active Queries:</span>
					{query.map(({ Tag: { name } }) => (
						<Tag
							key={name}
							onClick={() => {
								setQuery(
									R.reject(
										({ Tag: { name: name2 } }) =>
											name === name2,
										query,
									),
								);
							}}
						>
							{name}
						</Tag>
					))}
				</QueryRenderer>
			)}
		</React.Fragment>
	);
};

export default Topbar;
