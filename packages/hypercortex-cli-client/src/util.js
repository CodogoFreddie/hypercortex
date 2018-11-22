import { promisify } from "util";
import fs from "fs";
import openport from "openport";
import jsonfile from "jsonfile";
import envpaths from "env-paths";

export const rename = promisify(fs.rename);
export const stat = promisify(fs.stat);

export const getAPort = () =>
	new Promise((done, fail) => {
		openport.find({ startingPort: 5142 }, (err, port) =>
			err ? fail(err) : done(port),
		);
	});

export const readyGate = db => new Promise(done => db.on("ready", done));

export const createStateHandlers = type => {
	const getState = () =>
		jsonfile
			.readFile(envpaths(`hypercortex-${type}`).config)
			.catch(() => ({}));

	const setState = async newState => {
		const oldState = await getState();

		const updatedState = {
			...oldState,
			...newState,
		};

		await jsonfile.writeFile(
			envpaths(`hypercortex-${type}`).config,
			updatedState,
		);

		return updatedState;
	};

	return {
		getState,
		setState,
	};
};
