import React from 'react';
import {updateToDoItemCall} from "../api/update";
import { deleteToDoItemCall } from "../api/delete";
import {TaskStatus} from "../interfaces/toDoItems";


interface ToDoItemProps {
    title: string;
    id: number;
    passBackResponse: (response: any) => void;
    buttonMessage: string;
}


export const ToDoItem: React.FC<ToDoItemProps> = ({ title, id, passBackResponse, buttonMessage }) => {
    const sendRequest = async (): void => {
        if (buttonMessage === "edit") {
            await updateToDoItemCall(
                title,
                TaskStatus.DONE,
                id
            ).then(
                response => {
                    if (response.data) {
                        passBackResponse(response.data);
                    }
                    else if (response.error) {
                        console.log(response);
                        console.log(`Error ${response.status}: ${response.error}`);
                    }
                }
            )
        } else {
            await deleteToDoItemCall(
                title
            ).then(
                response => {
                    if (response.data) {
                        passBackResponse(response.data);
                    }
                    else if (response.error) {
                        console.log(response);
                        console.log(`Error ${response.status}: ${response.error}`);
                    }
                }
            )
        }
    };

    return (
        <div className="itemContainer" id={id}>
            <p>{title}</p>
            <button className="actionButton" onClick={sendRequest}>{buttonMessage}</button>
        </div>
    );
}
