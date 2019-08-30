import React from "react";

import HypertaskContext from "../HypertaskContext";

const Task = ({ task: { description } }) => {
	return <div>{description}</div>;
};

const TaskList = () => {
	const { tasks } = React.useContext(HypertaskContext);

	return tasks.map(({ score, task }) => (
		<Task key={task.id} task={task} score={score} />
	));
};

export default TaskList;
