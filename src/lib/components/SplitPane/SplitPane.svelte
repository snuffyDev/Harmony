<script lang="ts">
	import Handle from "./Handle";

	export let direction: "row" | "column" = "row";
	export let size = 2;

	let innerWidth = 640,
		innerHeight = 480;
	let x = 700,
		y = 500;
	$: axis = direction === "row" ? x : y;
	$: panel_1 = (axis / (direction === "row" ? innerWidth : innerHeight)) * 100;
	$: panel_2 = Math.round(100 - (axis / (direction === "row" ? innerWidth : innerHeight)) * 100);
	function handleMove(event: CustomEvent<{ x: number; y: number }>) {
		const detail = event.detail;
		x = detail.x;
		y = detail.y;
	}
</script>

<svelte:window bind:innerWidth />
<div
	class="panel"
	style="{direction === 'row' ? 'width' : 'height'}: {direction === 'row' ? x : y}px"
>
	<slot name="a" />
</div>
<Handle
	kind={direction === "row" ? "y" : "x"}

	on:move={handleMove}
	x={direction === "row" ? x : 0}
	y={direction === "column" ? y : 0}
/>
<div class="panel" style="{direction === 'row' ? 'width' : 'height'}: {panel_2}%">
	<slot name="b" />
</div>
<div
	class="measure"
	bind:clientHeight={innerHeight}
	style="display:block; position:absolute; height: 100%; top:0; left:0;"
/>

<style src="./index.scss" lang="scss">
</style>
