import replicate from "./replicate";

const authOtherWriter = async (db, [key]) => {
	await new Promise(done => db.authorize(Buffer.from(key, "hex"), done));

	await replicate(db, key);
};

export default authOtherWriter;
