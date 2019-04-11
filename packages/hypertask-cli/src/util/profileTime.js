let lastTime = new Date();
const logTime = (...labels) => {
	if (process.env.LOG_TIME) {
		console.error(labels.join("::"), new Date().getTime() - lastTime.getTime());
	}

	lastTime = new Date();
};

export default logTime;
