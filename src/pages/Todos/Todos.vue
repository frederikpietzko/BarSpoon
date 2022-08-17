<script setup lang="ts">
import TodoItem from "./components/TodoItem.vue"
import { useTodoStore } from "./todoStore"

const { todos } = useTodoStore()
</script>

<template>
  <div class="flex flex-col items-center">
    <h1 class="text-3xl font-bold w-1/2 mb-5">Todos</h1>
    <ul>
      <TodoItem v-for="todo in todos" :key="todo.id" :todo="todo" />
    </ul>
  </div>
</template>

<style scoped lang="sass">
ul
  @apply h-full w-1/2 flex flex-col gap-5
</style>
