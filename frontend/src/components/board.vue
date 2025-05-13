<template>
    <div>
        <div class="main">
            <h1>{{ categoryName }}</h1>

            <post
                v-for="pst in posts"
                :key="pst.id"
                :content="pst.content"
                :reply="pst.reply"></post>
        </div>
    </div>
</template>

<script>
    import { ref } from "vue";
    import post from "./post.vue";

    const posts = ref([
        {
            id: 1,
            content:
                "Lorem ipsum dolor, sit amet consectetur adipisicing elit. Ullam quisquam consequatur doloremque doloribus cum similique eaque id! Quo laudantium, perspiciatis repellendus similique delectus explicabo reprehenderit. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem.",

            reply: { isReply: false, replyId: -1 },
        },
        {
            id: 2,
            content:
                "Dolor, sit amet consectetur adipisicing elit. Ullam quisquam consequatur doloremque doloribus cum similique eaque id! Quo laudantium, perspiciatis repellendus similique delectus explicabo reprehenderit. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem.",
            reply: { isReply: false, replyId: -1 },
        },
        {
            id: 3,
            content:
                "Ipsum dolor, sit amet consectetur adipisicing elit. Ullam quisquam consequatur doloremque doloribus cum similique eaque id! Quo laudantium, perspiciatis repellendus similique delectus explicabo reprehenderit. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem. Nisi qui optio maiores fugiat, perferendis veritatis natus cumque animi magnam eligendi officia. Possimus, autem porro labore reiciendis sunt quam unde perferendis laborum maiores voluptatem.",
            reply: { isReply: false, replyId: 2 },
        },
    ]);

    export default {
        name: "board",

        components: {
            post,
        },

        props: ["categoryName"],

        data() {
            return {
                posts: posts,
            };
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
