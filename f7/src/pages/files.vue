<template>
    <f7-page name="files">
        <!-- <f7-navbar title="Form" back-link="Back"></f7-navbar> -->
        <f7-block strong-ios outline-ios>
            <f7-breadcrumbs>
                <f7-breadcrumbs-item>
                    <f7-link icon-ios="f7:house_fill" icon-md="material:home" text="Home" />
                </f7-breadcrumbs-item>
                <f7-breadcrumbs-separator />
                <f7-breadcrumbs-item active>文件</f7-breadcrumbs-item>
            </f7-breadcrumbs>
        </f7-block>

        <div v-if="!guid" class="grid grid-cols-2 medium-grid-cols-4 grid-gap">
            <f7-link no-link-class v-for="i in 10" :key="i" :href="`/files/${i}/`">
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
        <div v-else>
            {{ guid }}
        </div>
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

const props = defineProps({
    f7route: Object,
    f7router: Object,
})
const guid = ref('')

onMounted(() => {
    guid.value = props.f7route.params?.guid
})
</script>