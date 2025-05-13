<template>
    <div :class="{ replyStyle: reply.isReply }" class="post-container">
        <img src="../assets/jiji.jpg" alt="" />
        <p class="post-content-text">
            {{ content }}
        </p>
    </div>
</template>

<script>
    export default {
        name: "post",

        props: {
            content: String,
            reply: {
                isReply: Boolean,
                replyId: Number,
            },
        },
    };
</script>

<style>
    .post-container {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;

        border: 2px solid black;
        border-radius: 15px;

        padding: 10px;
        margin: 10px 0;

        width: 40%;
        overflow: hidden;

        transition: all 200ms ease-in-out;
    }

    .post-container:hover {
        box-shadow: 1px 12px 22px -2px rgba(0, 0, 0, 0.25);
        -webkit-box-shadow: 1px 12px 22px -2px rgba(0, 0, 0, 0.25);
        -moz-box-shadow: 1px 12px 22px -2px rgba(0, 0, 0, 0.25);

        transform: scale(1.01);
    }

    .post-container > img {
        min-width: 100px;
        height: 100%;
        margin-right: 10px;

        border-radius: 5px;
    }

    .post-content-text {
        font-size: 0.9rem;
        word-wrap: break-word;
    }

    .replyStyle {
        margin-left: 5%;
    }
</style>
