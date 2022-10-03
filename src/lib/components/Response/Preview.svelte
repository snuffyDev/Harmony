<script lang="ts">
	import DropdownMenu from "../DropdownMenu";
	import Toolbar from "../Toolbar";
	import Item from "../Toolbar/Item";
	import JSONPreview from "./JSONPreview";
	import PlaintextPreview from "./PlaintextPreview";
	import RawPreview from "./RawPreview";
	import WebPreview from "./WebPreview";

	export let type: "raw" | "web" | "plaintext" | "json" = "plaintext";
	export let data: any;

	type PreviewType = "raw" | "web" | "plaintext" | "json";
	interface $$Props<T extends PreviewType = PreviewType> {
		type: T;
		data: any;
	}

	const PREVIEWS: Record<
		PreviewType,
		typeof RawPreview | typeof JSONPreview | typeof PlaintextPreview | typeof WebPreview
	> = {
		raw: RawPreview,
		json: JSONPreview,
		plaintext: PlaintextPreview,
		web: WebPreview,
	};
</script>

<div class="preview">
	<svelte:component this={PREVIEWS[type]} {data} />
</div>

<style lang="scss">
	.preview {
		margin: 0 auto;
		padding: 0.5em;
		max-width: 100%;
		// aspect-ratio: 16/9;
		height: 100%;
		overflow: auto;
		background-color: var(--c-app-overlays-base-2);
		width: 100%;
	}
</style>
