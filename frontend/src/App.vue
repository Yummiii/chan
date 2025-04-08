<template>
    <div>
        <h1>Forum Social</h1>
    </div>
</template>

<script>
    export default {
        data() {
            return {
                activePage: 0,
                pages: [
                    {
                        link: { text: "Home", url: "index.html" },
                        pageTitle: "Pagina principal",
                        content: "",
                    },
                    {
                        link: { text: "About", url: "about.html" },
                        pageTitle: "Pagina sobre",
                        content: "",
                    },
                ],
            };
        },
    };
</script>
