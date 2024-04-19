import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";

// // @ts-ignore
// import styles from './index.module.css';
// import * as wasm from 'rust_interface';
// import init, { rust_add } from '../rust_interface/pkg/rust_interface';


const App = () => {

    const [data, setData] = useState(null);

    // on load
    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll<ToDoItems>(window.location.href);
            setData(response.data);
        }
        fetchData();
    }, []);

    return (
        <div>
            {data ? <div>Data loaded: {JSON.stringify(data)}</div> : <div>Loading...</div>}
        </div>
    );
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
