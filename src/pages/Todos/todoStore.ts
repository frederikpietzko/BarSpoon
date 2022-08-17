import { STATUS_CODES } from "http"
import { defineStore } from "pinia"
import { ref, watch } from "vue"
import { Todo } from "./model"

export const useTodoStore = defineStore("todos", {
  state: () => ({
    searchTerm: "",
    _todos: [
      {
        id: "1",
        title: "Learn Vue",
        completed: false,
      },
      {
        id: "2",
        title: "Learn Vuex",
        completed: false,
      },
      {
        id: "3",
        title: "Learn Vue Router",
        completed: true,
      },
    ] as Todo[],
  }),
  actions: {
    addTodo(todo: Todo) {
      this._todos.push(todo)
    },
    removeTodo(todo: Todo) {
      this._todos = this._todos.filter((t) => t.id !== todo.id)
    },
    toggleTodo(todo: Todo) {
      this._todos = this._todos.map((t) => {
        if (t.id === todo.id) {
          t.completed = !t.completed
        }
        return t
      })
    },
  },
  getters: {
    todos: (state) =>
      state._todos.filter(
        (t) =>
          t.title.includes(state.searchTerm) ||
          t.description?.includes(state.searchTerm)
      ),
  },
})
