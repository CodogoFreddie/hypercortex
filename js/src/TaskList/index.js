import React from "react";
import styled from "@emotion/styled";
import * as R from "ramda";
import { createShadow, calm } from "@freddieridell/little-bonsai-styles";

import HypertaskContext from "../HypertaskContext";

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

const caplitalizeFirst = R.pipe(
	R.split(""),
	R.over(R.lensIndex(0), x => x.toUpperCase()),
	R.join(""),
);

const Task = ({ task: { description, tags } }) => {
	return (
		<TaskStyled>
			<TaskDescription>{caplitalizeFirst(description)}</TaskDescription>
			{tags && (
				<TaskTags>
					{tags.map(tag => (
						<TaskTag key={tag}>{tag}</TaskTag>
					))}
				</TaskTags>
			)}
		</TaskStyled>
	);
};

const TaskList = () => {
	const { tasks } = React.useContext(HypertaskContext);

	return tasks.map(({ score, task }) => (
		<Task key={task.id} task={task} score={score} />
	));
};

export default TaskList;
