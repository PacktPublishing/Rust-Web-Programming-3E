import { ToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { deleteCall } from "./utils";


export async function deleteToDoItemCall(baseUrl: string, name: string) {
    let url = `${baseUrl}api/v1/delete/${name}`;
    return deleteCall<ToDoItems>(url, 200);
}
