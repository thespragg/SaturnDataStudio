import Connections from "@/views/connections/Connections.vue"
import { createRouter, createWebHistory } from "vue-router"

const routes = [
  { path: '/', component: Connections },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router;