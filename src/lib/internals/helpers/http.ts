import { get } from "$lib/cmd";

export class HttpClient {
	constructor(private name: string) {}

	async send(
		url: string,
		{ method = "GET", follow_redirects = false }: { method: "GET" | "POST"; follow_redirects: boolean },
	) {
		switch (method) {
			case "GET":
				return await get({ url, followRedirects: follow_redirects });

				case "POST":
				return await get({ url, followRedirects: follow_redirects });
		}
	}
}
