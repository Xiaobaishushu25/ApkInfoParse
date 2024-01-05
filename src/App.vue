<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import {onMounted, ref} from 'vue'
import { open } from "@tauri-apps/api/dialog";
import {invoke} from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import {ApkInfo} from "./apk_info.ts";
import ApkInfoCp from "./components/ApkInfoCp.vue";
// export type apk_info = {
//   package_name:string,
//   version_code: string,
//   version_name: string,
//   min_sdk_version: string,
//   target_sdk_version: string,
//   compile_sdk_version: string,
//   compile_sdk_version_code_name: string,
//   permissions: string[],
//   icon: string,
// }

const apkInfoList = ref<ApkInfo[]>([])
onMounted(async () => {
  // await appWindow.onResized(({payload: size}) => {
  await listen('tauri://file-drop', event => {
    console.log(event.payload[0]) //event.payload[0]就是path
    handleFiles([event.payload[0]])
  })
  // await appWindow.onFileDropEvent(({payload: fileDropEvent}) => {
  //   console.log("chufalema")
  //   console.log( fileDropEvent)
  // })
})
// function highlight() {
//   container.classList.add("highlight");
// }
// function unhighlight() {
//   this.$refs.container.classList.remove("highlight");
// }
// function handleDrop(e) {
//   // this.unhighlight();
//   console.log("进来了")
//   let dataTransfer = e.dataTransfer;
//   const files = Array.from(e.dataTransfer.files);
//   handleFiles(files);
// }
// function handleDragover(e){
//   e.preventDefault();
// }
function handleFiles(files:string[]) {
  // 处理文件逻辑
  console.log(files);
  for (let path of files){
    invoke<ApkInfo>('parse_by_path',{path:path}).then((response)=>{
      apkInfoList.value.push(response)
    }).catch((err)=>{
      console.log(err)
    })
  }
}
async function openFileChoose(){
  let files = await open({
    title:"选择apk",
    multiple:true,
    filters:[{
      name:"apk",
      extensions:["apk","apk.1"]
    }]
  })
  console.log(`选择pdf${files}`)
  if (files!=null){
    files = Array.from(files);
    handleFiles(files)
  }
}
function clearChoose(){
  apkInfoList.value.splice(0)
}
</script>

<template>
  <div class="container" @drop.prevent @dragover.prevent @dragenter.prevent>
    <div class="button-container">
      <button class="select-button" @click="openFileChoose">选择apk</button>
      <button class="select-button" @click="clearChoose">清除所有</button>
    </div>
    <div class="apk-info-container">
      <ApkInfoCp v-for="(apkInfo, index) in apkInfoList" :key="index" :apkInfo="apkInfo" />
    </div>
  </div>
</template>

<style>
.container {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 0;
}

.button-container {
  display: flex;
  gap: 10px; /* 调整两个按钮之间的间距 */
  height: 50px;
  margin: 0;
}

.select-button {
  margin:0; /* 通过 margin-bottom 添加 button 和 apk-info-container 之间的间隔 */
}

.apk-info-container {
  display: flex;
  flex-wrap: wrap;
  gap: 10px; /* 调整每个 ApkInfoCp 之间的间距 */
  margin-top: 20px;
}

.ApkInfoCp {
  width: 400px; /* 固定宽度 */
}
</style>