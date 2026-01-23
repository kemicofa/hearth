<script lang="ts">

    interface Props {
        name: string;
        id?: string;
    }

    let {
        name,
        id
    }: Props = $props();

    
</script>

<input type="text" {id} {name} />