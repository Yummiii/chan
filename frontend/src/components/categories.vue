<template>
    <div>
        <ul class="categ-list">
            <li v-for="route in routes" :key="route.id">
                <a :href="'/' + route.name"> {{ route.name }}</a>
            </li>
        </ul>
    </div>
</template>

<script>
    export default {
        name: "categories",

        props: ["routes"],
    };
</script>

<style>
    .categ-list {
        font-size: 1.2rem;
    }
</style>
