<script>
import '@fortawesome/fontawesome-free/css/all.min.css'
  import { onMount } from "svelte";

  let darkMode = false;

  onMount(() => {
    const initialTheme = window.matchMedia("prefers-color-scheme: dark")
      ? "dark"
      : "light";

    document.body.classList += initialTheme;

    darkMode = initialTheme === "dark";
  });

  function flip() {
    const currentTheme = darkMode ? "dark" : "light";

    darkMode = !darkMode;

    document.body.classList.replace(currentTheme, darkMode ? "dark" : "light");
  }
</script>

<button id="svelte" on:click={flip} role="switch" aria-checked={darkMode}>
  <i class={darkMode ? "fa-solid fa-sun" : "fa-solid fa-moon"} />
</button>

<style>
  button {
    box-sizing: content-box;
    padding: 0px;
    margin: 0;
    border: none;
    font-size: 2rem;
    background-color: transparent;
	display: inline-flex;
  }
</style>
