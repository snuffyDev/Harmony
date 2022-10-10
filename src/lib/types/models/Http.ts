import type { StatusCode } from "../http";

export interface HttpResponse {
	body: Uint8Array;
	status_code: StatusCode;
	content_type: string;
	response_time: number | undefined;
}

export interface Http