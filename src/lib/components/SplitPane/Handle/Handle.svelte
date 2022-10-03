<script lang="ts">
	import { createEventDispatcher } from "svelte";

	export let kind: "x" | "y" = "x";

	export let x = 0;
	export let y = 0;
	export let tag = "";

	const dispatch = createEventDispatcher<{ move: { x: number; y: number } }>();

	function dragHandler(node: HTMLElement) {
		function handleDown(event: PointerEvent) {
			event.stopPropagation();
			if (kind === "x") {
				y = event.pageY;
			}
			if (kind === "y") {
				x = event.pageX;
			}
			dispatch("move", { x, y });
			window.addEventListener("pointermove", handleMove, { passive: true });
			window.addEventListener("pointercancel", handleUp);
			window.addEventListener("pointerup", handleUp);
		}
		function handleMove(event: PointerEvent) {
			event.stopPropagation();

			if (kind === "x") {
				y = event.pageY;
			}
			if (kind === "y") {
				x = event.pageX;
			}
			dispatch("move", { x, y });
		}
		function handleUp(event: PointerEvent) {
			event.stopPropagation();

			if (kind === "x") {
				y = event.pageY;
			}
			if (kind === "y") {
				x = event.pageX;
			}

			dispatch("move", { x, y });
			window.removeEventListener("pointermove", handleMove);
			window.addEventListener("pointercancel", handleUp);
			window.removeEventListener("pointerup", handleUp);
		}
		node.addEventListener("pointerdown", handleDown);
		return {
			destroy() {
				node.removeEventListener("pointerdown", handleDown);
			},
		};
	}
</script>

<div
	class="handle {kind}"
	id={tag}
	use:dragHandler
	style="--{kind === 'y' ? 'x' : 'y'}: {kind === 'y' ? x : y}px;"
/>

<style src="./index.scss" lang="scss">
</style>
