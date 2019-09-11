import React from "react";
import styled from "@emotion/styled";
import * as R from "ramda";
import { createShadow, calm } from "@freddieridell/little-bonsai-styles";

import Task from "../Task";

const TaskList = ({ tasks, setQuery, query }) => {
	return tasks.map(({ score, task }) => (
		<Task
			key={task.id}
			task={task}
			score={score}
			setQuery={setQuery}
			query={query}
		/>
	));
};

export default TaskList;
