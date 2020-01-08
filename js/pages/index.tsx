import * as React from "react";
import Router from "next/router";

async function testSync() {
	const { sync_local_store_with_server } = await import(
		"@freddieridell/hypertask_sync_js_daemon"
	);

	try {
		const result = await sync_local_store_with_server({
			sync_secret:
				"heFKicyG3KDcRjQzQ5BNhA74k5MDJr3CsPlyitwCqAOeT0Ia1gsbfpiTa8Gbe4kH",
			server_url: "http://server.freddieridell.com:6346",
		});

		console.log({ result });
	} catch (e) {
		console.error(e);
	}
}

if (process.browser) {
	testSync();
}

const Home = () => {
	return <React.Fragment />;
};

export default Home;
