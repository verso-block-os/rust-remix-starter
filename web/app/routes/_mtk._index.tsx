import { Plus, X } from "lucide-react";
import { useReducer, useState } from "react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import type { MetaFunction } from "@remix-run/node";
import { ModeToggle } from "@/components/mode-toggle";
import { RiLogoutBoxLine } from "react-icons/ri";
import { Todo } from "@/generated/bindings";
import { api } from "@/lib/api";
import { useLoaderData } from "@remix-run/react";

export const meta: MetaFunction = () => {
  return [
    { title: "Rust Remix Starter" },
    { name: "description", content: "Welcome to Rust Remix Starter!" },
  ];
};

export const loader = async () => {
  const version = await api.query(["version"]);
  const todos = await api.query(["todos.getTodos"]);
  return {
    version,
    todos,
  };
};

type Action =
  | { type: "add"; payload: Todo }
  | { type: "remove"; payload: number }
  | { type: "toggle"; payload: number };

export default function Index() {
  const data = useLoaderData<typeof loader>();
  const [input, setInput] = useState("");
  const [todos, dispatch] = useReducer((state: Todo[], action: Action) => {
    switch (action.type) {
      case "add":
        return [...state, action.payload];
      case "remove":
        return state.filter((todo) => todo.id !== action.payload);
      case "toggle":
        return state.map((todo) => {
          if (todo.id === action.payload) {
            return { ...todo, completed: !todo.completed };
          }
          return todo;
        });
      default:
        return state;
    }
  }, data.todos);

  const createTodo = async () => {
    if (!input) return;

    const todo = await api.mutation(["todos.createTodo", input]);
    setInput("");
    dispatch({ type: "add", payload: todo });
  };

  const toggleTodo = async (id: number) => {
    const todo = await api.mutation(["todos.toggleTodo", id]);
    dispatch({ type: "toggle", payload: todo.id });
  };

  const deleteTodo = async (id: number) => {
    await api.mutation(["todos.deleteTodo", id]);
    dispatch({ type: "remove", payload: id });
  };

  return (
    <div className="flex-1 flex justify-center items-center relative">
      <div className="flex flex-col gap-4">
        <h1 className="text-xl font-bold">
          Welcome to Rust Remix Starter {data.version}
        </h1>
        <div className="flex gap-4">
          <Input
            type="text"
            placeholder="Add a todo"
            onChange={(e) => setInput(e.target.value)}
            value={input}
          />
          <div className="flex gap-4">
            <Button onClick={createTodo} size="icon">
              <Plus />
            </Button>
            <ModeToggle />
          </div>
        </div>
        {todos.map((todo) => (
          <div
            key={todo.id}
            className="flex items-center justify-between gap-4 p-4 bg-muted rounded-sm"
          >
            <div className="flex items-center gap-4">
              <input
                type="checkbox"
                checked={todo.completed}
                onChange={() => toggleTodo(todo.id)}
                className="w-4 h-4 rounded border border-gray-400"
              />
              <span className={todo.completed ? "line-through" : ""}>
                {todo.title}
              </span>
            </div>
            <Button
              onClick={() => deleteTodo(todo.id)}
              variant="ghost"
              size="icon"
            >
              <X />
            </Button>
          </div>
        ))}
        <Button
          className="gap-2"
          onClick={() => {
            // api.mutation(["auth.login", { email: "test", password: "test" }]);
            api.query(["auth.logout"]);
          }}
        >
          Logout <RiLogoutBoxLine />
        </Button>
      </div>
    </div>
  );
}
