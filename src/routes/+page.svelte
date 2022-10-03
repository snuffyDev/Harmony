<script lang="ts">
	import { get } from "$lib/cmd";
	import type { HttpResponse } from "$lib/types/http";
	import { onMount } from "svelte";
	import Layout from "$lib/components/Layout";
	import { HttpClient } from "$lib/internals/helpers/http";
	import { response_store } from "$lib/stores/response";
	import Button from "$lib/components/Button";
	import Sidebar from "$lib/components/Sidebar";
	let data: HttpResponse;
	let json: object;

	let url = "";
	let type_idx = 0;
	const TYPES = ["raw", "plaintext", "web", "json"];
	$: type = TYPES[type_idx];
	const http = new HttpClient("Client");
	let time = 0;
	async function send() {
		if (!url.match(/https?:\/\//)) url = "https://" + url;
		const startTime = Date.now();
		const res = await http.send(url, { method: "GET", follow_redirects: true });
		time = Date.now() - startTime;
		$response_store.original = res.body;
		if (type === "raw") {
			$response_store.data = res.body;
			return;
		}
		const utf8 = new TextDecoder().decode(new Uint8Array(res.body));
		switch (type) {
			case "json":
				$response_store.data = JSON.parse(utf8);
				break;
			default:
				$response_store.data = utf8.replace("<head>", `<head><base href="${url}">`);
		}

		$response_store.response = { ...res, body: undefined };
		console.log(http, data);
	}

	function changeType() {
		if (type_idx !== TYPES.length - 1) {
			type_idx += 1;
			return;
		}
		type_idx = 0;
	}
</script>

<Layout {type}>
	<main slot="main">
		<div class="container">
			<section>
				<div class="form">
					<input type="text" bind:value={url} placeholder="https://example.com/" />
					<Button on:click={changeType}>{type}</Button>
				</div>
			</section>

			<Button block on:click={send}>Send</Button>
		</div>
	</main>
</Layout>

<style lang="scss">
	main {
		padding: 1em;
		max-width: 95%;
		margin: 0 auto;
	}
	.form {
		display: flex;
		gap: 0.33333em;
	}
	section {
		margin-bottom: 1.333em;
	}
</style>
