import React from "react";

export function configHasBeenSet() {
	return !!localStorage.apiURL;
}

const Config = () => {
	const [url, setUrl] = React.useState(localStorage.apiURL);

	return (
		<div>
			input the url that you'll make API calls to
			<form>
				<input
					value={url}
					onChange={e => {
						setUrl(e.target.value);
					}}
				/>
				<button
					type="submit"
					onClick={() => {
						localStorage.apiURL = url;
						window.location = window.location;
					}}
				>
					submit
				</button>
			</form>
		</div>
	);
};

export default Config;
