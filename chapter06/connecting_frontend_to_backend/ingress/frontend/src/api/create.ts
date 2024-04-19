import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { postCall } from "./utils";


export async function createToDoItemCall(baseUrl: string, title: string) {
    const toDoItem: ToDoItem = {
        title: title,
        status: TaskStatus.PENDING
    };
    let url = `${baseUrl}api/create`;
    return postCall<ToDoItem, ToDoItems>(url, toDoItem, 201);
}
