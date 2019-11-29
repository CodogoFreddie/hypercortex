import React from "react";

import Editor from "../src/components/Editor";
import "../src/theme.css";

export default function HomePage({ query }) {
	return <Editor query={query} />;
}

HomePage.getInitialProps = async ({ query }) => ({
	query,
});
