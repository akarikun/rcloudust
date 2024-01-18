<template>
    <f7-page name="file-list">
        <frame>
            <div class="grid grid-cols-2 medium-grid-cols-4 grid-gap">
                <f7-link no-link-class v-for="i in 10" :key="i" :href="`/file-detail/${i}/`">
                    <f7-card outline-md class="demo-card-header-pic">
                        <f7-card-header valign="bottom"
                            style="background-image: url(https://cdn.framework7.io/placeholder/nature-1000x600-3.jpg)">Journey
                            To
                            Mountains</f7-card-header>
                        <f7-card-content>
                            <p class="date">Posted on January 21, 2015</p>
                        </f7-card-content>
                    </f7-card>
                </f7-link>
            </div>
        </frame>
    </f7-page>
</template>
<style lang="less">
.demo-card-header-pic {
    .card-header {
        height: 10vw;
        background-size: cover;
        background-position: center;
        color: #fff;
    }
}
</style>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import frame from './../components/frame.vue';

const props = defineProps({
    f7route: Object,
    f7router: Object,
})
const guid = ref('')

onMounted(() => {
    guid.value = props.f7route.params?.guid
        
})
</script>