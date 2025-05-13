/* eslint-disable */
<template>
    <categories :routes="routes"></categories>
    <board categoryName="Filmes"></board>
</template>

<script>
    import { ref } from "vue";
    import board from "./components/board.vue";
    import categories from "./components/categories.vue";

    let routes = ref([
        {
            id: 1,
            name: "Livros",
        },
        {
            id: 2,
            name: "Filmes",
        },
        {
            id: 3,
            name: "Musica",
        },
        {
            id: 4,
            name: "Programação",
        },
    ]);

    export default {
        components: {
            categories,
            board,
        },

        data() {
            return {
                routes: routes,
            };
        },
    };
</script>
<style></style>
