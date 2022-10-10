import { invoke } from "@tauri-apps/api";
import type { HttpGET, HttpPOST } from "./types/http";
import type { HttpResponse } from './types/models/Http'
interface Commands {
	get: {
		url: string;
		followRedirects: boolean;
	};
	post: {
		url: string;
		body: Uint8Array;
		headers: Record<string, string>;
		content_type: string;
	};
}

type Command<T extends keyof Commands = keyof Commands> = Commands[T];

const cmd = <Key extends keyof Commands = keyof Commands>(cmd: Key, args: Command<Key>) => invoke(cmd, args);

export const get = (args: Command<HttpGET>) => cmd("get", args) as Promise<unknown> as Promise<HttpResponse>;

export const post = (args: Command<HttpPOST>) => cmd("post", args) as Promise<unknown> as Promise<HttpResponse>;
