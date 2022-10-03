<script lang="ts">
	import { objKeys } from "$lib/utils/objects/keys";
	import { typeOf } from "$lib/utils/typeof";

	export let data: Record<string, any>;
	export let depth = 0;
	export let current_depth = 0;
	export let last = true;

	let items: (string | number)[];
	let isArray: boolean;
	let openBracket = "";
	let closeBracket = "";
	let collapsedSymbol = "...";
	$: {
		items = typeOf(data) === "object" ? objKeys(data) : [];
		isArray = Array.isArray(data);
		openBracket = isArray ? "[" : "{";
		closeBracket = isArray ? "]" : "}";
	}
	let collapsed = false;
	$: collapsed = depth < current_depth;

	const clicked = () => {
		collapsed = !collapsed;
	};

	const format = (item: unknown) => {
		switch (typeOf(item)) {
			case "string":
				return `"${item}"`;
			case "function":
				return "f () {...}";
			case "symbol":
				return (item as Symbol).toString();
			default:
				return item;
		}
	};
</script>

{#if !items.length}
	<span class="bracket" tabindex="0">{openBracket}{closeBracket}</span>{#if !last}<span
			class="comma">,</span
		>{/if}
{:else}
	<span class:hidden={collapsed}>
		<span class="bracket" on:click={clicked} tabindex="0">{openBracket}</span>
		<ul>
			{#each items as i, idx}
				<li>
					{#if !isArray}
						<span class="key">"{i}":</span>
					{/if}
					{#if typeOf(data[i]) === "object"}
						<svelte:self
							data={data[i]}
							{depth}
							current_depth={current_depth + 1}
							last={idx === items.length - 1}
						/>
					{:else}
						<span class="val {typeOf(data[i])}"
							>{format(data[i])}{#if idx < items.length - 1}<span class="comma">,</span>{/if}</span
						>
					{/if}
				</li>
			{/each}
		</ul>
		<span class="bracket" on:click={clicked} tabindex="0">{closeBracket}</span>{#if !last}<span
				class="comma">,</span
			>{/if}
	</span>
	<span class="bracket" class:hidden={!collapsed} on:click={clicked} tabindex="0"
		>{openBracket}{collapsedSymbol}{closeBracket}</span
	>{#if !last && collapsed}<span class="comma">,</span>{/if}
{/if}

<style src="./index.scss" lang="scss">
</style>
