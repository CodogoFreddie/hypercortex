import * as React from "react";

const TextInput: React.FC<{
	id: string;
	kind: "text" | "password";
	label: string;
	value: string;
	onChange: (val: string) => void;
}> = ({ id, kind, label, value, onChange }) => {
	return (
		<React.Fragment>
			<label htmlFor={id}>{label}</label>
			<input
				id={id}
				value={value}
				onChange={e => onChange(e.target.value)}
				type={kind}
			/>
		</React.Fragment>
	);
};

export default TextInput;
