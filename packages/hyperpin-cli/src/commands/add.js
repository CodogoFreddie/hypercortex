import * as R from "ramda";
import cheerio from "cheerio";

import getId from "@hypercortex/easy-type-id";

import renderPin from "../util/renderPin";

const add = async ({pin}, _ ,url) => {
	const newID = getId(16);
	const newPin = pin(newID);

	const response = await fetch(url);
	const raw = await response.text();

	const $ = cheerio.load(raw);

	await newPin.createdAtSet(new Date().toISOString());
	await newPin.urlSet(url);
	await newPin.authorSet($("meta[property='article:author']").attr("content"));
	await newPin.canonicalSet($("link[rel='canonical']").attr("href"));
	await newPin.descriptionSet($("meta[name='description']").attr("content"));
	await newPin.iconSet($("link[rel='icon']").attr("sizes"));
	await newPin.imageSet($("meta[property='og:image']").attr("content"));
	await newPin.keywordsSet(($("meta[name='keywords']").attr("content")||"").split(",").slice(0, 5));
	await newPin.siteNameSet($("meta[property='og:site_name']").attr("content"));
	await newPin.subjectSet($("meta[name='subject']").attr("content"));
	await newPin.titleSet($("title").text());

	const renderedPin = await renderPin(newPin);
	console.log(renderedPin);
};

export default add;
