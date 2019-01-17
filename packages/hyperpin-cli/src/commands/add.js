import * as R from "ramda";
import cheerio from "cheerio";

import getId from "@hypercortex/easy-type-id";

//import renderPin from "../util/renderPin";

const dateTimeProps = new Set(["due", "wait", "sleep", "snooze"]);

const add = async (url, pin) => {
	const newID = getId(16);
	const newPin = pin(newID);

	const response = await fetch(url);
	const raw = await response.json();

	const $ = cheerio.load(raw);
	console.log($("head"));
};

export default add;
