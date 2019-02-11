import winSize from "window-size";

import renderPin from "../util/renderPin";

const basicDisplay = async ({ pinAll }) => {
	let linesPrinted = 0;
	const { height } = winSize.get();

	for (const pin of await pinAll()) {
		const renderedPin = await renderPin(pin);
		linesPrinted = linesPrinted + renderedPin.split("\n").length;
		if (linesPrinted <= height) {
			console.log(renderedPin);
		} else {
			return;
		}
	}
};

export default basicDisplay;
