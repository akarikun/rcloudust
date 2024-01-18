<template>
    <div>
        <!-- <f7-navbar title="Form" back-link="Back"></f7-navbar> -->
        <f7-block strong-ios outline-ios>
            <f7-breadcrumbs>
                <f7-breadcrumbs-item>
                    <f7-link :href="`/`" icon-ios="f7:house_fill" icon-md="material:home" text="主页" />
                </f7-breadcrumbs-item>
                <f7-breadcrumbs-separator />
                <f7-breadcrumbs-item active>
                    <f7-link :href="`/file-list/`" icon-ios="f7:house_fill" icon-md="material:file" text="文件" />
                </f7-breadcrumbs-item>
            </f7-breadcrumbs>
        </f7-block>
        <slot></slot> 
    </div>
</template>
<script>

</script>