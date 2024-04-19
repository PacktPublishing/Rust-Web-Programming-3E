import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { putCall } from "./utils";


export async function updateToDoItemCall(baseUrl: string, name: string, status: TaskStatus) {
    const toDoItem: ToDoItem = {
        title: name,
        status: status
    };
    let url = `${baseUrl}api/v1/update`;
    return putCall<ToDoItem, ToDoItems>(url, toDoItem, 200);
}