<script setup lang="ts">
defineProps({message:String})
</script>

<template>
  <div class="custom-alert">
    <div class="content">
      <p>{{ message }}</p>
<!--      <button @click="closeAlert">Close</button>-->
    </div>
  </div>
</template>

<style scoped>
.custom-alert {
  position: fixed;
  top: 2%;
  left: 50%;
  transform: translate(-50%, -50%);
  padding: 8px;
  background-color: #ef0a0a;
  color: white;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  font-size: 17px; /* Adjust font size as needed */
  line-height: 1.5; /* Adjust line height as needed */
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>