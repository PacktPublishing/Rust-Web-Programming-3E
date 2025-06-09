// File: frontend/src/api/get.ts
import { ToDoItems } from "../interfaces/toDoItems";
import {getCall} from "./utils";
import { Url } from "./url";


export default async function getAll() {
    let response = await getCall<ToDoItems>(
        new Url().getAll, 
        200
    );
    return response;
}
