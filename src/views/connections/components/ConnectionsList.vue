<template>
  <div class="w-full flex flex-col justify-center items-center">
    <h3 class="text-white p-2 text-lg">Your Connections</h3>
    <div class="w-full cursor-pointer">
      <p v-for="connection in connections" :key="connection.id" class="p-2 text-gray-900 text-center bg-gray-200 hover:bg-gray-300 border-b border-gray-400 border-solid">
        {{ connection.name }}
      </p>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { PostgresConnection } from "@/types/PostgresConnection";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

const connections = ref<PostgresConnection[]>([]);
invoke("get_connections").then(
  (res) => (connections.value = res as PostgresConnection[])
);
</script>
