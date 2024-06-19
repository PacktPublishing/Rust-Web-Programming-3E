import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { putCall } from "./utils";
import { Url } from "./url";


export async function updateToDoItemCall(name: string, status: TaskStatus, id: number) {
    const toDoItem: ToDoItem = {
        title: name,
        status: status,
        id: id
    };
    return putCall<ToDoItem, ToDoItems>(new Url().update, toDoItem, 200);
}
