import React from "react";

import Document, { Html, Head, Main, NextScript } from "next/document";

class MyDocument extends Document {
	static async getInitialProps(ctx) {
		const initialProps = await Document.getInitialProps(ctx);
		return { ...initialProps };
	}

	render() {
		return (
			<Html>
				<Head>
					<link
						rel="stylesheet"
						href="https://assets.littlebonsai.co.uk/theme.css"
					/>
					<link
						rel="stylesheet"
						href="https://assets.littlebonsai.co.uk/reset.css"
					/>
				</Head>
				<body data-theme="system-inverted">
					<Main />
					<NextScript />
				</body>
			</Html>
		);
	}
}

export default MyDocument;
