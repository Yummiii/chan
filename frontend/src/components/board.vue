<template>
    <div>
        <div class="main">
            <textarea
                v-model="postTextContent"
                placeholder="Escreva sua postagem..."
                rows="4"
                cols="50"></textarea>
            <button id="post-btn" @click="addPost(postTextContent)">
                Add Post
            </button>
            <input
                type="file"
                name="img"
                accept="image/x-png,image/gif,image/jpeg" />

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

    let posts = ref([
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

        methods: {
            addPost(text) {
                if (text && text.length < 1000) {
                    let newPost = {
                        id: 4,
                        content: text,
                        reply: { isReply: false, replyId: -1 },
                    };

                    posts.value.push(newPost);
                } else {
                    alert("Por favor insira uma entrada válida");
                }
            },
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
