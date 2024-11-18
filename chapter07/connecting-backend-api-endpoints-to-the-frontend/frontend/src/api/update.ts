import { ToDoItem, ToDoItems, TaskStatus } 
from "../interfaces/toDoItems";
import { putCall } from "./utils";
import { Url } from "./url";

export async function updateToDoItemCall(
    name: string, status: TaskStatus) {
    const toDoItem: ToDoItem = {
        title: name,
        status: status
    };
    return putCall<ToDoItem, ToDoItems>(
        new Url().update, 
        toDoItem, 
        200
    );
}
