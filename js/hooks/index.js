import React from "react";

export function useLocalStorageState(key, initial) {
	const theLocalStorage = process.browser ? localStorage : {};

	const createInitial = () =>
		JSON.stringify(typeof initial === "function" ? initial() : initial);
	const getFromStorage = () =>
		JSON.parse(theLocalStorage[key] || createInitial());

	const [state, setState] = React.useState(getFromStorage);

	React.useEffect(() => {
		setState(getFromStorage());
	}, []);

	React.useEffect(() => {
		theLocalStorage[key] = JSON.stringify(state);
	}, [state]);

	return [state, setState];
}
