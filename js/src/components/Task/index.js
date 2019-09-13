import React from "react";
import styled from "@emotion/styled";
import * as R from "ramda";
import { createShadow, calm } from "@freddieridell/little-bonsai-styles";
import toDate from "date-fns/fp/toDate";
import parseISO from "date-fns/fp/parseISO";
import format from "date-fns/fp/format";
import formatDistanceWithOptions from "date-fns/fp/formatDistanceWithOptions";

const TaskStyled = styled.div(
	calm({
		display: "flex",
		flexDirection: "column",
		padding: R.path(["theme", "size", "space", 2]),
	}),
);

const TaskDescription = styled.h2(
	calm({
		borderBottomColor: R.path(["theme", "color", "symantic", "text"]),
		borderBottomStyle: "solid",
		borderBottomWidth: R.path(["theme", "size", "space", 1]),
		color: R.path(["theme", "color", "symantic", "text"]),
		fontSize: R.path(["theme", "size", "font", 3]),
		fontWeight: "normal",
		margin: 0,
		padding: R.path(["theme", "size", "space", 2]),
	}),
);

const TaskTags = styled.ul(
	calm({
		display: "flex",
		margin: 0,
		padding: 0,
		margin: R.path(["theme", "size", "space", 2]),
	}),
);

const TaskTag = styled.li(
	calm({
		marginLeft: R.path(["theme", "size", "space", 5]),
	}),
);

const TaskDate = styled.div(calm({}));

const caplitalizeFirst = R.pipe(
	R.split(""),
	R.over(R.lensIndex(0), x => x.toUpperCase()),
	R.join(""),
);

const formatDate = R.pipe(
	R.tap(console.log),
	parseISO,
	R.tap(console.log),
	R.when(
		Boolean,
		R.converge(R.append, [
			R.always([]),
			format("yyyy-MM-dd BBB"),
			formatDistanceWithOptions({ addSuffix: true }, new Date()),
		]),
	),
);

const Task = ({ task: { description, tags, due }, setQuery, query }) => {
	return (
		<TaskStyled>
			<TaskDescription>{caplitalizeFirst(description)}</TaskDescription>
			{due && <TaskDate>{formatDate(due)}</TaskDate>}
			{tags && (
				<TaskTags>
					{tags.map(tag => (
						<TaskTag
							key={tag}
							onClick={() => {
								setQuery([
									...query,
									{
										Tag: {
											sign: "Plus",
											name: tag,
										},
									},
								]);
							}}
						>
							{tag}
						</TaskTag>
					))}
				</TaskTags>
			)}
		</TaskStyled>
	);
};

export default Task;
