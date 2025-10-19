import React, { useState } from 'react';
import { createToDoItemCall } from "../api/create";
// import "../App.css";

// Define the props type, if there are additional props you can add them here
interface CreateToDoItemProps {
    passBackResponse: (response: any) => void; // Define the response type more specifically if possible
}

export const CreateToDoItem: React.FC<CreateToDoItemProps> = ({ passBackResponse }) => {
    const [title, setTitle] = useState<string>("");

    const createItem = async () => {
        await createToDoItemCall(title).then(response => {
                setTitle("");
                if (response.data) {
                    passBackResponse(response.data);
                } else if (response.error) {
                    console.log(response);
                    console.log(`Error ${response.status}: ${response.error}`);
                }
            });
    };

    const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setTitle(e.target.value);
    };

    return (
        <div className="inputContainer">
            <input type="text" id="name"
                   placeholder="create to do item"
                   value={title}
                   onChange={handleTitleChange}/>
            <button className="actionButton"
                 id="create-button"
                 onClick={createItem}>Create</button>
        </div>
    );
}

