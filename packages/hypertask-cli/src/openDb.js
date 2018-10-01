import * as R from "ramda";
import envpaths from "env-paths";
import fs from "fs";
import hyperdb from "hyperdb";
import util from "util";
import inquirer from "inquirer";
import jsonfile from "jsonfile";

import replicate from "./replicate";

const openDb = async () => {
	const { data: dataPath, config: configPath } = envpaths("hypercortex");

	try {
		//already initalised the db, load it and return
		const config = await jsonfile.readFile(configPath);

		const db = hyperdb(dataPath, {
			valueEncoding: "json",
		});

		await new Promise((done, fail) => {
			db.on("ready", done);
		});

		return db;
	} catch (e) {
		console.log("no existing hypercortex detected");

		const { key } = await inquirer.prompt([
			{
				name: "key",
				message:
					"Existing hypercortex key (leave blank to create a new hypercortex)",
			},
		]);

		if (key.length) {
			//use an existing hypercortex

			console.log(`joining hypercortex   "${key}"`);

			const db = hyperdb(dataPath, Buffer.from(key, "hex"), {
				valueEncoding: "json",
			});

			await new Promise((done, fail) => {
				db.on("ready", done);
			});

			console.log(`you local key is "${db.local.key.toString("hex")}"`);
			console.log(
				"please authorise this key on another client by executing",
			);
			console.log(`    $ task auth ${db.local.key.toString("hex")}`);
			console.log("to complete setup");

			replicate(db);

			await new Promise(done => {
				setInterval(
					() =>
						db.authorized(db.local.key, (err, authed) => {
							if (authed) {
								done();
							}
						}),
					1000,
				);
			});

			console.log("authorised");

			await jsonfile.writeFile(configPath, {
				hypercortex: {
					key,
				},
			});

			return db;
		} else {
			//create a new hypercortex
			const db = hyperdb(dataPath, {
				valueEncoding: "json",
			});

			await new Promise((done, fail) => {
				db.on("ready", done);
			});

			console.log(`your hypercortex key is "${db.key.toString("hex")}"`);

			await jsonfile.writeFile(configPath, {
				hypercortex: {
					key: db.key.toString("hex"),
				},
			});

			return db;
		}
	}
};

export default openDb;
