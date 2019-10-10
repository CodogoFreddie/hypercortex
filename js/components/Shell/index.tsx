import * as React from "react";

const Shell: React.FC<{
	children: React.ReactNode;
	title: string;
}> = ({ children, title }) => {
	return (
		<React.Fragment>
			<h1>{title}</h1>
			{children}
		</React.Fragment>
	);
};

export default Shell;
