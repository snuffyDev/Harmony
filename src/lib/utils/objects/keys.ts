export function objKeys<
	T extends object = object,
	Keys extends { [PropertyKey in keyof T]: T[PropertyKey] } = { [PropertyKey in keyof T]: T[PropertyKey] },
>(obj: T) {
	if (typeof obj !== "object") return [];

	const keys: (keyof Keys)[] = [];
	for (const key in obj) {
		keys.push(key);
	}
	return keys;
}
