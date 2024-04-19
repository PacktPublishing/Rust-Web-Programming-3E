import { ToDoItems } from "../interfaces/toDoItems";
import {getCall} from "./utils";

export default async function getAll(baseUrl: string) {
    let response = await getCall<ToDoItems>(`${baseUrl}api/v1/get/all`, 200);
    return response;
}
