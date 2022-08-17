import { createApp } from "vue"
import "./style.sass"
import App from "./App.vue"
import { createPinia } from "pinia"
import { createRouter, createWebHistory } from "vue-router"
import routes from "./routes"

const pinia = createPinia()
const router = createRouter({
  history: createWebHistory(),
  routes,
})
const app = createApp(App)

app.use(pinia)
app.use(router)
app.mount("#app")
