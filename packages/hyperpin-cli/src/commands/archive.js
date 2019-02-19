import renderPin from "../util/renderPin";

const archive = async ({ pin }, _, id) => {
	const pinToArchive = pin(id);

	await pinToArchive.archivedSet(true);

	const renderedPin = await renderPin(pinToArchive);
	console.log(renderedPin);
};

export default archive;
