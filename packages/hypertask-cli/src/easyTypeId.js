const chars = "fjdkslaghtyrueiwoqpnbmvcxz6574839201";

const p = 0.1;

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

export default getEasyToTypeId;
