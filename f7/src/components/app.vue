<template>
  <f7-app v-bind="f7params">

    <!-- Left panel with cover effect when hidden -->
    <f7-panel left cover dark :visible-breakpoint="960">
      <f7-view>
        <f7-page>
          <f7-navbar title="rcloudust"></f7-navbar>
          <!-- <f7-block-title>Left View Navigation</f7-block-title>
          <f7-list>
            <f7-list-item link="/left-page-1/" title="Left Page 1"></f7-list-item>
            <f7-list-item link="/left-page-2/" title="Left Page 2"></f7-list-item>
          </f7-list> -->
          <!-- <f7-block-title>菜单管理</f7-block-title> -->
          <f7-list>
            <!-- <f7-list-item link="/about/" view=".view-main" panel-close title="About"></f7-list-item>
            <f7-list-item link="/form/" view=".view-main" panel-close title="Form"></f7-list-item>
            <f7-list-item link="#" view=".view-main" back panel-close title="Back in history"></f7-list-item> -->
            <f7-list-item link="/" view=".view-main" title="主页"></f7-list-item>
            <f7-list-item link="/file-list/" view=".view-main" title="文件"></f7-list-item>
            <f7-list-item link="/share/" view=".view-main" title="分享"></f7-list-item>
          </f7-list>
        </f7-page>
      </f7-view>
    </f7-panel>


    <!-- Right panel with reveal effect-->
    <!-- <f7-panel right reveal dark>
      <f7-view>
        <f7-page>
          <f7-navbar title="Right Panel"></f7-navbar>
          <f7-block>Right panel content goes here</f7-block>
        </f7-page>
      </f7-view>
    </f7-panel> -->
    <f7-view main class="safe-areas" url="/"></f7-view>
  </f7-app>
</template>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';


import routes from '../js/routes.js';
import store from '../js/store';

const f7params = ref({
  name: 'rcloudust', // App name
  theme: 'auto', // Automatic theme detection
  // App store
  store: store,
  // App routes
  routes: routes,
  view: {
    browserHistory: true,
    browserHistorySeparator: ''
  }
});
const username = ref('');
const password = ref('');


onMounted(() => {
  f7ready(() => {
    // Call F7 APIs here
    console.log(f7.colors);
  });
});
const alertLoginData = () => {
  f7.dialog.alert('Username: ' + username.value + '<br>Password: ' + password.value, () => {
    f7.loginScreen.close();
  });
}
</script>