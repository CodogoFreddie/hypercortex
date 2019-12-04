import React from "react";

import Editor from "../src/components/Editor";

export default function HomePage({ query }) {
	return <Editor query={query} />;
}

HomePage.getInitialProps = async ({ query }) => ({
	query,
});
