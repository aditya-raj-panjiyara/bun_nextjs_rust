"use client"
import { useState, useEffect } from "react";
import Image from "next/image";
import axios from "axios";

interface Todo {
  id: string;
  todo: string;
  done: boolean;
  created_at: string;
  updated_at: string;
}

const API_BASE_URL = "http://localhost:8001";

// Create axios instance with default config
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export default function Home() {
  const [input, setInput] = useState<string>("");
  const [todos, setTodos] = useState<Todo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>("");

  // Fetch all todos
  const fetchTodos = async () => {
    try {
      setLoading(true);
      const response = await api.get<Todo[]>("/todos");
      setTodos(response.data);
      setError("");
    } catch (err) {
      setError("Failed to load todos. Make sure the Rust server is running on port 8001.");
      console.error("Error fetching todos:", err);
    } finally {
      setLoading(false);
    }
  };

  // Add new todo
  const addTodo = async () => {
    if (!input.trim()) return;

    try {
      setLoading(true);
      await api.post<Todo>("/todos", {
        todo: input.trim(),
        done: false,
      });

      setInput("");
      await fetchTodos(); // Refresh the list
      setError("");
    } catch (err) {
      setError("Failed to add todo");
      console.error("Error adding todo:", err);
    } finally {
      setLoading(false);
    }
  };

  // Toggle todo completion
  const toggleTodo = async (todo: Todo) => {
    try {
      setLoading(true);
      await api.put<Todo>(`/todos/${todo.id}`, {
        todo: todo.todo,
        done: !todo.done,
      });

      await fetchTodos(); // Refresh the list
      setError("");
    } catch (err) {
      setError("Failed to update todo");
      console.error("Error updating todo:", err);
    } finally {
      setLoading(false);
    }
  };

  // Delete todo
  const deleteTodo = async (todoId: string) => {
    try {
      setLoading(true);
      await api.delete(`/todos/${todoId}`);

      await fetchTodos(); // Refresh the list
      setError("");
    } catch (err) {
      setError("Failed to delete todo");
      console.error("Error deleting todo:", err);
    } finally {
      setLoading(false);
    }
  };

  // Load todos on component mount
  useEffect(() => {
    fetchTodos();
  }, []);

  // Handle Enter key press
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      addTodo();
    }
  };

  return (
    <div className="font-sans grid grid-rows-[20px_1fr_20px] items-center justify-items-center min-h-screen p-8 pb-20 gap-16 sm:p-20">
      <h1 className="flex items-center gap-3 text-5xl font-bold">
        Todo App with RUST
        <Image width={100} height={100} alt={"rust happy"} src={"./rustacean-flat-happy.svg"} />
        backend
      </h1>

      <div className="flex flex-col gap-4 w-full max-w-[500px]">
        {/* Error message */}
        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
            {error}
          </div>
        )}

        {/* Add todo form */}
        <div className="flex gap-4">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            className="border border-gray-400 rounded w-full py-2 px-3 focus:outline-none focus:border-blue-500"
            placeholder="Enter a todo item"
            disabled={loading}
          />
          <button
            onClick={addTodo}
            disabled={loading || !input.trim()}
            className="border border-gray-400 rounded py-2 px-4 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? "..." : "Add"}
          </button>
        </div>

        {/* Refresh button */}
        <button
          onClick={fetchTodos}
          disabled={loading}
          className="self-start border border-blue-400 text-blue-600 rounded py-1 px-3 hover:bg-blue-50 disabled:opacity-50"
        >
          {loading ? "Loading..." : "Refresh"}
        </button>

        {/* Todo list */}
        <div className="flex flex-col gap-2">
          {todos.length === 0 && !loading && (
            <p className="text-gray-500 text-center py-8">
              No todos yet. Add one above!
            </p>
          )}

          {todos.map((todo) => (
            <div
              key={todo.id}
              className="flex items-center gap-3 p-3 border border-gray-200 rounded hover:bg-gray-50"
            >
              <input
                type="checkbox"
                checked={todo.done}
                onChange={() => toggleTodo(todo)}
                className="w-4 h-4"
                disabled={loading}
              />
              <span
                className={`flex-1 ${todo.done ? 'line-through text-gray-500' : ''}`}
              >
                {todo.todo}
              </span>
              <button
                onClick={() => deleteTodo(todo.id)}
                disabled={loading}
                className="text-red-500 hover:text-red-700 px-2 py-1 text-sm disabled:opacity-50"
              >
                Delete
              </button>
            </div>
          ))}
        </div>

        {/* Todo count */}
        {todos.length > 0 && (
          <div className="text-sm text-gray-600 text-center">
            {todos.filter(t => !t.done).length} of {todos.length} todos remaining
          </div>
        )}
      </div>
    </div>
  );
}
