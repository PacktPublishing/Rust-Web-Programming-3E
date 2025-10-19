import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import { LoginForm } from "./components/LoginForm";
import "./App.css";

import { CreateToDoItem } from './components/CreateItemForm';
import init, { rust_generate_button_text } from '../rust-interface/pkg/rust_interface.js'

const App = () => {

    const [data, setData] = useState(null);
    const [wasmReady, setWasmReady] = useState<boolean>(false);
    const [RustGenerateButtonText, setRustGenerateButtonText] = useState<(input: string) => string>(null);
    const [loggedin, setLoggedin] = useState<boolean>(
        localStorage.getItem('token') !== null
    );

    React.useEffect(() => {
        init().then(() => {
            setRustGenerateButtonText(() => rust_generate_button_text);
            setWasmReady(true);
        }).catch(e => console.error(
            "Error initializing WASM: ", e
        ));
    }, []);
    
    function setToken(token: string) {
        localStorage.setItem('token', token);
        setLoggedin(true);
    }

    function reRenderItems(items: ToDoItems) {
        setData(items);
    }

    // on load
    React.useEffect(() => {
        const fetchData = async () => {
            // Check if wasmReady is true before fetching data
            if (wasmReady && loggedin) {
                const response = await getAll<ToDoItems>();
                setData(response.data);
            }
        };

        // Only fetch data if wasmReady is true
        if (wasmReady) {
            fetchData();
        }
    }, [wasmReady, loggedin]); // to fire when wasmReady changes

    if (localStorage.getItem('token') === null) {
        return <LoginForm setToken={setToken} />;
    }

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
                                    buttonMessage={RustGenerateButtonText(item.status)}
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
                                    buttonMessage={RustGenerateButtonText(item.status)}
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
