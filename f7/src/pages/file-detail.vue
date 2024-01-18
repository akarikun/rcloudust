<template>
    <f7-page name="file-detail">
        <frame>
            <div class="img-container">
                <img src="https://cdn.framework7.io/placeholder/nature-1000x600-3.jpg" />
            </div>
        </frame>
    </f7-page>
</template>
<style lang="less">
    .img-container {
        text-align: center;
        img {
            display:inline-block;
        }
    }
</style>
<script setup>
import frame from './../components/frame.vue';
</script>