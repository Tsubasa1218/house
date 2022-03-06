<script>
	import { onMount } from "svelte";
	let canvas;

	onMount(() => {
		const ctx = canvas.getContext("2d");

		const { width, height } = canvas;

		ctx.fillStyle = "#f00";

		const size = 5;
		const spacing = 5;

		for (let i = 1; i <= 20; i++) {
			ctx.fillRect(i * (size + spacing), height - 10, size, size);
		}
	});
</script>

<figure>
	<ul>
		<li>30</li>
		<li>20</li>
		<li>10</li>
		<li>0</li>
	</ul>
	<canvas bind:this={canvas}>Canvas not supported</canvas>
</figure>

<style>
	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	figure {
		display: flex;
		margin: 0;
	}

	input {
		background-color: var(--bg-color);
		border: 1px solid var(--font-color);
		color: var(--font-color);
		border-radius: 0;
	}
</style>
