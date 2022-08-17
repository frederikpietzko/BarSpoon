import { def } from "@vue/shared"
import { RouteRecordRaw } from "vue-router"

const routes: RouteRecordRaw[] = [
  { path: "/", component: () => import("./pages/Todos") },
]

export default routes
