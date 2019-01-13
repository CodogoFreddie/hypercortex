import hyperswarm from "@hyperswarm/network";

const share = ({ db }) => {
	console.log("sharing hypercortex");

	const swarm = hyperswarm({ ephemeral: true });
	swarm.on("connection", (socket, { client, peer }) => {
		if (client) {
			console.log(`connected to peer ${peer.host}:${peer.port}`);
		} else {
			console.log(`connected to someone`);
		}

		socket.pipe(db.replicate({ live: false })).pipe(socket);
	});
	swarm.join(db.discoveryKey, {
		lookup: true,
		announce: true,
	});
};

export default share;
