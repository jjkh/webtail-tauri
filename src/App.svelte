<script lang="ts">
	import { open } from "@tauri-apps/api/dialog";
	import { invoke } from "@tauri-apps/api/tauri";
	import FilterScroll from "./FilterScroll.svelte";

	let filterInput: string = undefined;
	let filterRegExp: RegExp = undefined;
	$: {
		if (filterInput) {
			try {
				filterRegExp = new RegExp(filterInput);
			} catch (error) {
				filterRegExp = undefined;
			}
		} else {
			filterRegExp = undefined;
		}
	}
	let filename: string = undefined;
	let fileError: string = undefined;
	let lines: string[] = [];

	async function openDialog() {
		let openResult = await open();
		if (typeof openResult !== "string") {
			return; // probably empty dialog i guess
		}

		filename = openResult;
		lines = [];
		fileError = undefined;

		invoke("test_interop", { filename: openResult })
			.then((l: string[]) => (lines = l))
			.catch((err) => (fileError = err));
	}
</script>

<FilterScroll filterSelector=".filtered">
	<main>
		<h1>WebTail</h1>
		<div>
			<button on:click="{openDialog}"> Open file... </button>
			<span>Filename: {filename || "No file selected!"}</span>
		</div>

		{#if fileError}
			<p class="error">{fileError}</p>
		{:else if lines.length > 0}
			<h3>Filter: <input bind:value="{filterInput}" /></h3>
			<div class="lines">
				{#each lines as line}
					<pre
						class="line"
						class:filtered="{filterRegExp &&
							filterRegExp.test(line)}">{line}</pre>
				{/each}
			</div>
		{:else}
			<h3>No file selected!</h3>
		{/if}
	</main>
</FilterScroll>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
	}

	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	.lines {
		text-align: left;
	}

	.line {
		background: #aaaaaa;
		padding: 0 4px;
		margin: 0;
	}

	.line.filtered {
		background: #ffaaaa;
	}
</style>
