import { ToDoItem, ToDoItems, TaskStatus } 
from "../interfaces/toDoItems";
import { postCall } from "./utils";
import { Url } from "./url";


export async function createToDoItemCall(title: string) {
    const toDoItem: ToDoItem = {
        title: title,
        status: TaskStatus.PENDING
    };
    return postCall<ToDoItem, ToDoItems>(
        new Url().create, 
        toDoItem, 
        201
    );
}
