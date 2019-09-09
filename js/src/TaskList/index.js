import React from "react";
import styled from "@emotion/styled";
import * as R from "ramda";
import { createShadow, calm } from "@freddieridell/little-bonsai-styles";

import HypertaskContext from "../HypertaskContext";

import Task from "../Task";

const TaskList = () => {
	const { tasks, runCommand } = React.useContext(HypertaskContext);

	return tasks.map(({ score, task }) => (
		<Task key={task.id} task={task} score={score} runCommand={runCommand} />
	));
};

export default TaskList;
