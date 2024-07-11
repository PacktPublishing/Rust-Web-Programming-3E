import { NewToDoItem, ToDoItems, TaskStatus } from "../interfaces/toDoItems";
import { postCall } from "./utils";
import { Url } from "./url";


export async function createToDoItemCall(title: string) {
    const toDoItem: NewToDoItem = {
        title: title,
        status: TaskStatus.PENDING
    };
    return postCall<NewToDoItem, ToDoItems>(new Url().create, toDoItem, 201);
}
