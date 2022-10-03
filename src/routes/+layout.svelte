<div id="app-wrapper">
	<slot />
</div>

<style global lang="scss">
	@import "../app.scss";

	html,
	body {
		font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu,
			Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
		min-height: 100vh;
		position: fixed;
		inset: 0;
		max-height: 100%;
	}
	#app-wrapper {
		position: absolute;
		isolation: isolate;
		inset: 0;
	}
</style>
