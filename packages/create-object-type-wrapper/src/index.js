import * as R from "ramda";

const chars = "fjdkslaghtyrueiwoqpnbmvcxz6574839201";

const p = 0.04;

const getEasyToTypeId = (n = 0, i = 0) => {
	if (n) {
		if (Math.random() < p) {
			return chars[i] + getEasyToTypeId(n - 1);
		} else {
			return getEasyToTypeId(n, (i + 1) % chars.length);
		}
	} else {
		return "";
	}
};

const resolveNodeConflict = R.pipe(
	R.reduce((l, r) => (l.modifiedAt > r.modifiedAt ? l : r)),
	R.prop("value"),
);

const createScalarHandlers = (type, scalars, db, id) =>
	scalars.map(prop => ({
		get [prop]() {
			return new Promise((done, fail) =>
				db.get(
					`/data/${type}/${id}/${prop}`,
					(err, dat) => (err ? fail(err) : done(dat)),
				),
			).then(resolveNodeConflict);
		},

		set [prop](value) {
			return new Promise((done, fail) =>
				db.set(
					`/data/${type}/${id}/${prop}`,
					{
						modifiedAt: new Date().toISOString(),
						modifiedBy: db.local.key.toString("hex"),
						value,
					},
					(err, dat) => (err ? fail(err) : done(dat)),
				),
			);
		},
	}));

const createCollectionHandlers = (type, collections, db, id) => {
	return collections.map(prop => ({
		get[prop]() {
			return new Promise( (done, fail) => done([]))
		}
		set[prop](value){
			throw `error setting ${prop} on type ${type}: can not directly assign to collections, please use ${prop}Add(x), and ${prop}Remove(x)`;
		}

		[`${prop}Add`]: input => {
			if(typeof input === "object"){
				return new Promise( (done, fail) => {
					const key = input.key || input.id;
					const value = {
						...val,
						key: key || getEasyToTypeId()
					}

					db.set(
						`/data/${type}/${id}/${prop}/${value.key}`,
						{
							modifiedAt: new Date().toISOString(),
							modifiedBy: db.local.key.toString("hex"),
							value,
						},
						(err, dat) => (err ? fail(err) : done(dat)),
					);
				})
			}
			else {
				const value = input;
					db.set(
						`/data/${type}/${id}/${prop}/${value}`,
						{
							modifiedAt: new Date().toISOString(),
							modifiedBy: db.local.key.toString("hex"),
							value,
						},
						(err, dat) => (err ? fail(err) : done(dat)),
					);
			}
		},
		[`${prop}Remove`]: val => new Promise((done, fail) => done()),
	}));
}

const createObjecTypeWrapper = R.curry(
	(type, { scalars, collections, relations: { one, many } }, db, id) =>
	Object.assign(
		{
			toObj: (depth = 0) => {},
		},
		...createScalarHandlers(type, scalars, db, id),
		...createCollectionHandlers(type, collections, db, id),
	),
);

export default createObjecTypeWrapper;
