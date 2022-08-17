<script setup lang="ts">
import CircleCheck from "assets/circle-check-solid.svg"
import ClockRotateLeft from "assets/clock-rotate-left-solid.svg"
import Trash from "assets/trash-solid.svg"
import { Todo } from "../model"
import { useTodoStore } from "../todoStore"

interface Props {
  todo: Todo
}

const { todo } = defineProps<Props>()

const { removeTodo, toggleTodo } = useTodoStore()
</script>
<template>
  <li>
    <span :class="todo.completed && 'completed'">
      {{ todo.title }}
    </span>
    <div class="flex gap-2">
      <button
        v-if="!todo.completed"
        class="w-6 text-slate-700 hover:shadow-md hover:text-slate-800"
        @click="toggleTodo(todo)"
      >
        <CircleCheck />
      </button>
      <button
        v-if="todo.completed"
        class="w-6 text-slate-700 hover:shadow-md hover:text-slate-800"
        @click="toggleTodo(todo)"
      >
        <ClockRotateLeft />
      </button>
      <button
        v-if="todo.completed"
        @click="removeTodo(todo)"
        class="w-6 text-slate-700 hover:shadow-md hover:text-slate-800"
      >
        <Trash />
      </button>
    </div>
  </li>
</template>
<style scoped lang="sass">

li
  @apply rounded ring-green-600 px-1 py-2 bg-gradient-to-l from-green-400 to-cyan-600 flex justify-between shadow hover:-mx-1 hover:shadow-lg hover:to-cyan-500
.completed
  @apply line-through decoration-cyan-300 decoration-2
</style>
