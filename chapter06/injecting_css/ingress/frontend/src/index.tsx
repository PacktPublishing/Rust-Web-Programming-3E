import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import "./App.css";

// // @ts-ignore
// import styles from './index.module.css';
// import * as wasm from 'rust_interface';
// import init, { rust_add } from '../rust_interface/pkg/rust_interface';
import { CreateToDoItem } from './components/CreateItemForm';

const App = () => {

    const [data, setData] = useState(null);

    function reRenderItems(items: ToDoItems) {
        setData(items);
    }

    // on load
    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll<ToDoItems>();
            setData(response.data);
        }
        fetchData();
    }, []);

    if (!data) {
        return <div>Loading...</div>;
    }
    else {
        return (
            <div className="App">
            <div className="mainContainer">
                <div className="header">
                    <p>complete tasks: {data.done.length}</p>
                    <p>pending tasks: {data.pending.length}</p>
                </div>
                <h1>Pending Items</h1>
                <div>
                    {data.pending.map((item, index) => (
                        <><ToDoItem key={item.title + item.status}
                                    title={item.title}
                                    status={item.status}
                                    id={item.id}
                                    passBackResponse={reRenderItems}/>
                        </>
                    ))}
                </div>
                <h1>Done Items</h1>
                <div>
                    {data.done.map((item, index) => (
                        <><ToDoItem key={item.title + item.status}
                                    title={item.title}
                                    status={item.status}
                                    id={item.id}
                                    passBackResponse={reRenderItems}/>
                        </>
                    ))}
                </div>
                <CreateToDoItem passBackResponse={reRenderItems} />
            </div>
            </div>
        );
    }
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
