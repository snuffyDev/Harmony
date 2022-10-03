<script lang="ts">
	import { response_store } from "$lib/stores/response";

	export let data: any;
</script>

<code>{JSON.stringify($response_store.original ?? {})}</code>

<style src="./index.scss" lang="scss">
</style>
