import * as R from "ramda";

const resolveNodeConflict = R.reduce(
	(l, r) => {
		if(!l.value){
			return r;
		} 
		
		if(!r.value){
			return l;
		}
		return l.value.modifiedAt > r.value.modifiedAt ? l : r;
	},
	{ value: { modifiedAt: "" } },
);

export default resolveNodeConflict;
