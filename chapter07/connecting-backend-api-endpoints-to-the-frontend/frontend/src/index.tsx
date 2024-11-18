// File: frontend/src/index.tsx
import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";

const App = () => {
    const [data, setData] = useState(null);
    const [error, setError] = useState(null); // State to store error messages

    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();

            if (response.error) {
                setError(response.error); // Set error if response contains an error
            } else {
                setData(response.data); // Set data if response is successful
            }
        };
        
        fetchData();
    }, []);

    return (
        <div>
            {error ? (
                <div style={{ color: 'red' }}>Error: {error}</div> // Display error if present
            ) : data ? (
                <div>Data loaded: {JSON.stringify(data)}</div>
            ) : (
                <div>Loading...</div>
            )}
        </div>
    );
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
