<script>
    import { addSiteToBlocklist } from '$lib/api';

    const inputs = {
        site: "",
    };

    async function submit() {
        const blockResult = await addSiteToBlocklist(inputs.site)
        console.log({ blockResult});
    }
</script>

<p>Add Site to List</p>

<form on:submit|preventDefault="{submit}">
    <input type="text" bind:value={inputs.site}>
    <button type="submit">Submit</button>
</form>
