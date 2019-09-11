import React from "react";

//ask for data persistence
export default function() {
	React.useEffect(() => {
		if (navigator.storage && navigator.storage.persist) {
			navigator.storage.persisted().then(persisted => {
				if (!persisted) {
					navigator.storage.persist();
				}
			});
		}
	}, []);
}
