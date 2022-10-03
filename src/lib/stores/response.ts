import { writable } from "svelte/store";

export const response_store = _response();
function _response() {
	const { subscribe, set, update } = writable<{ response: Record<string, any>; data: any; original: any }>({
		response: {},
		original: undefined,
		data: undefined,
	});

	return {
		subscribe,
		set,
	};
}
