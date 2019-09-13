import React from "react";
import ReactDOM from "react-dom";

const Modal = props => {
	const thisModalRef = React.useRef(document.createElement("div"));

	React.useEffect(() => {
		thisModalRef.current.style = [
			"bottom: 0;",
			"left: 0;",
			"position: absolute",
			"right: 0;",
			"top: 0;",
		].join(" ");

		const modalRoot = document.body;

		modalRoot.appendChild(thisModalRef.current);

		return () => {
			modalRoot.removeChild(thisModalRef.current);
		};
	}, []);

	return ReactDOM.createPortal(props.children, thisModalRef.current);
};

export default Modal;
