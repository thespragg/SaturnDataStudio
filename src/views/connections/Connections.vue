<template>
  <div class="w-full h-screen bg-zinc-900 flex justify-center items-center">
    <div class="w-1/3">
      <ConnectionsList />
      <div
        class="text-white cursor-pointer flex justify-center p-3"
        @click="showAddModal = true"
      >
        <span class="font-bold text-emerald-500">+</span> Add Connection
      </div>
    </div>
  </div>
  <NModal v-model:show="showAddModal">
    <NCard title="Add Connection" class="w-1/2">
      <NForm>
        <NFormItem label="Name">
          <NInput v-model:value="newConnection.name" placeholder="Connection name"/>
        </NFormItem>
        <NFormItem label="Connection">
          <NInput v-model:value="newConnection.conn" placeholder="IP/URL"/>
        </NFormItem>
        <NFormItem label="Username">
          <NInput v-model:value="newConnection.username" placeholder="Username"/>
        </NFormItem>
        <NFormItem label="Password">
          <NInput v-model:value="newConnection.password" placeholder="Password"/>
        </NFormItem>
      </NForm>
      <template #footer>
        <div class="flex justify-end">
          <NButton class="mr-2" @click="saveConnection" type="success">Save</NButton>
          <NButton @click="showAddModal = false" type="error">Close</NButton>
        </div>
      </template>
    </NCard>
  </NModal>
</template>

<script lang="ts" setup>
import ConnectionsList from "./components/ConnectionsList.vue";
import { NModal, NCard, NButton, NInput, NForm, NFormItem } from "naive-ui";
import { ref } from "vue";
import { PostgresConnection } from "@/types/PostgresConnection";
import { invoke } from "@tauri-apps/api";

const showAddModal = ref(false);
const newConnection = ref<PostgresConnection>({
  conn: "",
  name: "",
  username: "",
  password: ""
});

const saveConnection = () => {
  invoke("create_connection", newConnection.value).then(_ => showAddModal.value = false).catch(e=> console.log({e}));
}
</script>
