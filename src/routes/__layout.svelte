<script>
    import "normalize.css/normalize.css";

    import Nav from '$lib/components/Nav.svelte';
    import { onMount } from 'svelte';
    import { blocklist, fetchBlocklist } from '$lib/api';

    onMount(async () => {
        const result = await fetchBlocklist();
        blocklist.set(result)
    });
</script>

<Nav />

<slot></slot>
