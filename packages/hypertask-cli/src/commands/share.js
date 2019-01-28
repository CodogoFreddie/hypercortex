var createSwarm = require("discovery-swarm");
const portfinder = require("portfinder");
var defaults = require("dat-swarm-defaults");

const share = async ({ db }) => {
	console.log("sharing hypercortex");

	var swarm = createSwarm(
		defaults({
			utp: false,
			tcp: true,
			maxConnections: 3,
		}),
	);

	const port = await portfinder.getPortPromise();
	swarm.listen(port);
	swarm.join(db.discoveryKey); // can be any id/name/hash

	swarm.on("connection", (connection, { host, port }) => {
		console.log(`found + connected to peer ${host}:${port}`);
		connection.pipe(db.replicate({ live: false })).pipe(connection);
	});
};

export default share;
