import createSwarm from "hyperdiscovery";

const share = ({ db }) => {
	console.log("sharing hypercortex");

	const swarm = createSwarm(db, {
		upload: true,
		download: true,
	});

	swarm.on("connection", function(peer, { host, port }) {
		console.log(`connected to ${host}:${port}`);
		swarm.on("close", function() {
			console.log(`disconected from ${host}:${port}`);
		});
	});

	console.log("started sharing");
};

export default share;
