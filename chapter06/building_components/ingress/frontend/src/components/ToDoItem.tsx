import React, { useState, useEffect } from 'react';
import axios from "axios";
import "../App.css";
import { updateToDoItemCall } from "../api/update";
import { deleteToDoItemCall } from "../api/delete";


interface ToDoItemProps {
    title: string;
    status: string;
    id: number;
    passBackResponse: (response: any) => void;
    logout: () => void;
}


export const ToDoItem: React.FC<ToDoItemProps> = ({ title, status, id, passBackResponse, logout }) => {
    const [itemTitle, setTitle] = useState<string>(title);
    const [itemStatus, setStatus] = useState<string>(status);
    const [button, setButton] = useState<string>('');

    useEffect(() => {
        const processStatus = (status: string): string => {
            return status === "PENDING" ? "edit" : "delete";
        };

        setButton(processStatus(status));
    }, [status]);

    const inverseStatus = (status: string): string => {
        return status === "PENDING" ? "DONE" : "PENDING";
    };

    const sendRequest = (): void => {
        const headers = {
            headers: { "token": localStorage.getItem("token") }
        };
        const apiUrl = `${window.location.href}/v1/item`;

        if (button === "edit") {

            axios.post(apiUrl, { title: itemTitle, status: inverseStatus(itemStatus) }, headers)
                .then(response => passBackResponse(response))
                .catch(error => {
                    if (error.response?.status === 401) {
                        logout();
                    }
                });
        } else {
            axios.delete(`${apiUrl}/${id}`, headers)
                .then(response => passBackResponse(response))
                .catch(error => {
                    if (error.response?.status === 401) {
                        logout();
                    }
                });
        }
    };

    return (
        <div className="itemContainer">
            <p>{itemTitle}</p>
            <button className="actionButton" onClick={sendRequest}>{button}</button>
        </div>
    );
}
