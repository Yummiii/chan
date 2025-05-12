<template>
    <div class="main">
        <post></post>
        <post></post>
        <post></post>
    </div>
</template>

<script>
    import post from "./components/post.vue";
    export default {
        components: {
            post,
        },
        data() {
            return {};
        },
    };
</script>
<style>
    .main {
        display: flex;
        align-items: center;
        justify-content: center;

        flex-direction: column;
    }
</style>
