import React from "react";

import Editor from "../js/components/Editor";

export default function HomePage({ query }) {
	return <Editor query={query} />;
}

HomePage.getInitialProps = async ({ query }) => ({
	query,
});
