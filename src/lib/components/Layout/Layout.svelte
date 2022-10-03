<script lang="ts">
	import { response_store } from "$lib/stores/response";

	import Preview from "../Response/Preview.svelte";

	import SplitPane from "../SplitPane";
	import Toolbar from "../Toolbar";
	import Item from "../Toolbar/Item";

	export let type: any;
</script>

<Toolbar>
	<Item>TODO</Item>
</Toolbar>
<div class="app">
	<SplitPane size={2}>
		<div class="main" slot="a">
			<slot name="main" />
		</div>

		<div class="results" slot="b">
			<SplitPane size={2} direction="column">
				<Preview data={$response_store.data} {type} slot="a" />
				<div class="response" slot="b">
					<p>Response Info</p>
					<pre>{JSON.stringify($response_store.response)}</pre>
				</div>
			</SplitPane>
		</div>
	</SplitPane>
</div>

<style src="./index.scss" lang="scss">
</style>
