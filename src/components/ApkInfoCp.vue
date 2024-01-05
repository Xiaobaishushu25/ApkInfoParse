<script setup lang="ts">
import {PropType, ref} from 'vue'
import {ApkInfo} from "../apk_info.ts";
defineProps({apkInfo:Object as PropType<ApkInfo>})
const showPermissions = ref(false)
function togglePermissions() {
  showPermissions.value = !showPermissions.value
}
</script>

<template>
  <div class="app-info-container">
    <h2>{{apkInfo.file_name}}</h2>
    <ul class="zebra-list">
      <li><strong @click="togglePermissions">Package Name:</strong> {{ apkInfo.package_name }}</li>
      <li><strong>Version Code:</strong> {{ apkInfo.version_code }}</li>
      <li><strong>Version Name:</strong> {{ apkInfo.version_name }}</li>
      <li><strong>Min SDK Version:</strong> {{ apkInfo.min_sdk_version }}</li>
      <li><strong>Target SDK Version:</strong> {{ apkInfo.target_sdk_version }}</li>
      <li><strong>Compile SDK Version:</strong> {{ apkInfo.compile_sdk_version }}</li>
      <li><strong>Compile SDK Version Code Name:</strong> {{ apkInfo.compile_sdk_version_code_name }}</li>
      <li>
        <strong class="click" @click="togglePermissions">Permissions(click to view or hide details)</strong>
        <ul :class="{ 'hidden': !showPermissions }">
          <li v-for="(permission, index) in apkInfo.permissions" :key="index">{{ permission }}</li>
        </ul>
      </li>
      <li><strong>Icon:</strong> <img :src="apkInfo.icon" alt="App Icon" width="50" height="50"></li>
    </ul>
  </div>
</template>

<style>
body {
  font-family: 'Arial', sans-serif;
  background-color: #f2f2f2;
  margin: 20px;
}

.app-info-container {
  background-color: #eeb3df;
  border-radius: 10px;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
  padding: 20px;
  width: 400px;
}

ul {
  list-style-type: none;
  padding: 0;
}

.zebra-list li:nth-child(even) {
  background-color: #bdcec9;
}

li {
  margin-bottom: 8px; /* Reduced margin for a more compact look */
  padding: 8px; /* Reduced padding for a more compact look */
  border-radius: 5px;
  line-height: 1.0; /* Adjusted line height for a more compact look */
}

strong {
  color: #3498db;
}
.click {
  cursor: pointer;
}
img {
  border-radius: 5px;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.2);
}

.hidden {
  display: none;
}
</style>