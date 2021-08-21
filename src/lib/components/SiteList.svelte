<script>
    import { blocklist } from '$lib/api';
</script>

{#if $blocklist.length}
    <ul>
        {#each $blocklist as site}
            <li>{site}</li>
        {/each}
    </ul>
{/if}

<style>
  ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
  }
</style>