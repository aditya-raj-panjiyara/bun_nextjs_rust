#!/usr/bin/env python3
import requests
import json

BASE_URL = "http://localhost:8001"

def test_api():
    print("Testing Todo API...")
    
    # Test 1: Get all todos (should be empty initially)
    print("\n1. Getting all todos:")
    response = requests.get(f"{BASE_URL}/todos")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    
    # Test 2: Create a new todo
    print("\n2. Creating a new todo:")
    new_todo = {"todo": "Learn Rust", "done": False}
    response = requests.post(f"{BASE_URL}/todos", json=new_todo)
    print(f"Status: {response.status_code}")
    todo_data = response.json()
    print(f"Response: {todo_data}")
    todo_id = todo_data["id"]
    
    # Test 3: Get the specific todo
    print(f"\n3. Getting todo {todo_id}:")
    response = requests.get(f"{BASE_URL}/todos/{todo_id}")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    
    # Test 4: Update the todo
    print(f"\n4. Updating todo {todo_id}:")
    update_data = {"todo": "Learn Rust with Axum", "done": True}
    response = requests.put(f"{BASE_URL}/todos/{todo_id}", json=update_data)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    
    # Test 5: Get all todos again
    print("\n5. Getting all todos after update:")
    response = requests.get(f"{BASE_URL}/todos")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
    
    # Test 6: Delete the todo
    print(f"\n6. Deleting todo {todo_id}:")
    response = requests.delete(f"{BASE_URL}/todos/{todo_id}")
    print(f"Status: {response.status_code}")
    
    # Test 7: Verify deletion
    print("\n7. Getting all todos after deletion:")
    response = requests.get(f"{BASE_URL}/todos")
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")

if __name__ == "__main__":
    try:
        test_api()
    except requests.exceptions.ConnectionError:
        print("Error: Could not connect to the API. Make sure the server is running on http://localhost:8001")
    except Exception as e:
        print(f"Error: {e}")