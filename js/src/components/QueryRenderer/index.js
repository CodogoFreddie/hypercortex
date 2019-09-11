import React from "react";
import * as R from "ramda";

const QueryRenderer = ({ query, setQuery }) => {
	return (
		<div>
			<button
				onClick={() => {
					localStorage.removeItem("apiURL");
					window.location = window.location;
				}}
			>
				clear config
			</button>

			{query.map(({ Tag: { name } }) => (
				<div
					key={name}
					onClick={() => {
						setQuery(
							R.reject(
								({ Tag: { name: name2 } }) => name === name2,
								query,
							),
						);
					}}
				>
					{name}
				</div>
			))}
		</div>
	);
};

export default QueryRenderer;
