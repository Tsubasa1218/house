<script>
  import { onMount } from "svelte";

  let selected = false;

  onMount(() => {
    const initialTheme = window.matchMedia("prefers-color-scheme: dark")
      ? "dark"
      : "light";

    document.body.classList += initialTheme;

    selected = initialTheme === "dark";
  });

  function flip() {
    const currentTheme = selected ? "dark" : "light";

    selected = !selected;

    document.body.classList.replace(currentTheme, selected ? "dark" : "light");
  }
</script>

<button id="svelte" on:click={flip} role="switch" aria-checked={selected}>
  {selected ? "🌙" : "🔆"}
</button>

<style>
  button {
    box-sizing: content-box;
    padding: 0px;
    margin: 0;
    border: none;
    font-size: 2rem;
    background-color: transparent;
  }
</style>
