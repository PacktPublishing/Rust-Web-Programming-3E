import React, {useEffect, useState} from 'react';
// import "../App.css";
import {updateToDoItemCall} from "../api/update";
import { deleteToDoItemCall } from "../api/delete";
import {TaskStatus} from "../interfaces/toDoItems";


interface ToDoItemProps {
    title: string;
    status: string;
    id: number;
    passBackResponse: (response: any) => void;
}


export const ToDoItem: React.FC<ToDoItemProps> = ({ title, status, id, passBackResponse }) => {
    const [itemTitle, setTitle] = useState<string>(title);
    const [button, setButton] = useState<string>('');

    useEffect(() => {
        const processStatus = (status: string): string => {
            return status === "PENDING" ? "edit" : "delete";
        };
        setButton(processStatus(status));
    }, [status]);

    const sendRequest = async (): void => {
        if (button === "edit") {

            await updateToDoItemCall(
                itemTitle,
                TaskStatus.DONE
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
                itemTitle
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
            <p>{itemTitle}</p>
            <button className="actionButton" onClick={sendRequest}>{button}</button>
        </div>
    );
}
