<script lang="ts">
	export let outline = false;
	export let block = false;
</script>

<button class="button {outline ? 'outline' : ''}" class:block on:click>
	<slot />
</button>

<style src="./index.scss" lang="scss">
</style>
