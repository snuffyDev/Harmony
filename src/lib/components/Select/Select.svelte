<script lang="ts">
	import type { SelectOption } from "$lib/types/models/ui/select";

	let _class = "";

	export { _class as class };
	export let items: SelectOption[] = [];
	export let name = "";
	export let disabled = false;
</script>

<select {name} {disabled} class={_class}>
	{#each items as { disabled = false, label, value, selected = false }}
		<option {disabled} {label} {value} {selected}>{label}</option>
	{/each}
</select>

<style src="./index.scss" lang="scss">
</style>
