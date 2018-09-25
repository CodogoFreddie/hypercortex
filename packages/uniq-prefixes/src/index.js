import * as R from "ramda";

const generateUniqPrefixes = ids => {
	const root = {
		value: null,
		children: {},
	};

	const insert = (root, key, value) => {
		let node = root;
		let indexLastChar = null;

		for (const i in key) {
			const char = key[i];
			if (node.children[char]) {
				node = node.children[char];
			} else {
				indexLastChar = i;
				break;
			}
		}

		if (indexLastChar != null) {
			for (const char of key.slice(indexLastChar)) {
				node.children[char] = {
					value: null,
					children: {},
				};
				node = node.children[char];
			}
		}
		node.value = value;
	};

	const flatten = node => {
		const countChildren = ({ children, value }, n = 0) => {
			if (value) {
				return n + 1;
			} else if (children) {
				return R.pipe(
					R.values,
					R.map(countChildren),
					R.sum,
				)(children);
			} else {
				return n;
			}
		};

		const getOnlyChild = node => {
			if (node.value) {
				return node.value;
			}
			if (countChildren(node) === 1) {
				return getOnlyChild(R.values(node.children)[0]);
			}
		};

		if (countChildren(node) === 1) {
			return {
				value: getOnlyChild(node),
				children: {},
			};
		} else {
			return R.evolve({
				children: R.map(flatten),
			})(node);
		}
	};

	ids.forEach(id => insert(root, id, id));

	const flattenedTrie = flatten(root);

	const prefixes = {};

	const walkTrie = ({ children, value }, path = "") => {
		if (value) {
			prefixes[value] = path;
		}

		for (const char of R.keys(children)) {
			walkTrie(children[char], path + char);
		}
	};

	walkTrie(flattenedTrie);

	return prefixes;
};

export default generateUniqPrefixes;
