<script lang="ts">
	import { onMount } from "svelte";

	export let data: string;

	let iframe: HTMLIFrameElement;

	onMount(() => {});
</script>

<div class="preview-wrapper">
	<iframe referrerpolicy="same-origin" allow="" bind:this={iframe} srcdoc={data} frameborder="0" />
</div>

<style src="./index.scss" lang="scss">
</style>
