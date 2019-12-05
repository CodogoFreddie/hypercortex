import React from "react";
import cs from "classname";

import css from "./styles.scss";

export default function Labeled({
	id,
	label,
	children,
	classNameSection,
	classNameLabel,
	...props
}) {
	return (
		<section className={cs(css.section, classNameSection)}>
			<label
				className={cs(css.label, classNameLabel)}
				htmlFor={id}
				{...props}
			>
				{label}
			</label>
			{children}
		</section>
	);
}
