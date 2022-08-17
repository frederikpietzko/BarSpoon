<script setup lang="ts">
import { ref } from "vue"
import AnglesLeft from "./assets/angles-left-solid.svg"
import AnglesRight from "./assets/angles-right-solid.svg"

const open = ref(true)
</script>

<template>
  <div class="flex relative">
    <div
      class="w-5 text-lg absolute top-2 left-3 cursor-pointer"
      @click="open = !open"
    >
      <AnglesRight v-if="open" />
      <AnglesLeft v-else />
    </div>
    <Transition>
      <nav class="navbar" v-if="!open">
        <RouterLink class="link" to="/">Todos</RouterLink>
      </nav>
    </Transition>
    <div class="py-10 px-3 w-full h-screen">
      <RouterView></RouterView>
    </div>
  </div>
</template>

<style scoped lang="sass">
.navbar
  animation: moveToRight .8 cubic-bezier(0.36, 0.07, 0.19, 0.97) both
  @apply flex flex-col w-1/6 px-3 py-10 text-lg border-r-2 rounded-3xl border-green-400 h-screen
  .link
    @apply py-2 px-1 rounded bg-gradient-to-r from-cyan-600 to-green-400 text-slate-100 shadow hover:shadow-lg hover:from-cyan-500 hover:to-green-400

.v-enter-active
  transition: all 0.3s ease-out
.v-leave-active
  transition: all 0.6s cubic-bezier(.17,.67,.83,.67)

.v-enter-from, .v-leave-to
  transform: translateX(-70px)
  opacity: 0
</style>
