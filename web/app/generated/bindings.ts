// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "auth.logout", input: never, result: null } | 
        { key: "auth.verify", input: string, result: VerifiedUser } | 
        { key: "todos.getTodos", input: never, result: Todo[] } | 
        { key: "version", input: never, result: string },
    mutations: 
        { key: "auth.login", input: AuthArgs, result: null } | 
        { key: "auth.register", input: AuthArgs, result: null } | 
        { key: "todos.createTodo", input: string, result: Todo } | 
        { key: "todos.deleteTodo", input: number, result: null } | 
        { key: "todos.toggleTodo", input: number, result: Todo },
    subscriptions: never
};

export type AuthArgs = { email: string; password: string }

export type Todo = { id: number; title: string; completed: boolean }

export type VerifiedUser = { id: number; email: string }
