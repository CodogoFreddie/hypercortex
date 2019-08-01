import React from "react";

const HypertaskContext = React.createContext({
	tasks: {},
	runCommand: () => {},
});

export default HypertaskContext;
