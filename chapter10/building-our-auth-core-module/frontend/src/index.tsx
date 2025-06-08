// File: frontend/src/index.tsx
import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";
import { CreateToDoItem } from './components/CreateItemForm';
import "./App.css";
import init, { rust_generate_button_text } from '../rust-interface/pkg/rust_interface.js';



const App = () => {
    const [data, setData] = useState(null);
    const [error, setError] = useState(null);
    const [wasmReady, setWasmReady] = useState<boolean>(false);
    const [
        RustGenerateButtonText, 
        setRustGenerateButtonText
    ] = useState<(input: string) => string>(null);


    function reRenderItems(response: any) {
        if (response.error) {
            alert(JSON.stringify(response));
            return;
        }
        else if (response.data) {
            setData(response.data);
            setError(null);
        }
        else {
            setError("Unknown error");
        }
    }

    React.useEffect(() => {
        init().then(() => {
            setRustGenerateButtonText(() => rust_generate_button_text);
            setWasmReady(true);
        }).catch(e => console.error(
            "Error initializing WASM: ", e
        ));
    }, []);    

    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll();
            if (response.error) {
                setError(response.error); // Set error if response contains an error
            } else {
                setData(response.data); // Set data if response is successful
            }
        };
        if (wasmReady) {
            fetchData();
        }
    }, [wasmReady]);

    if (error) {
        return <div style={{ color: 'red' }}>Error: {error}</div>
    }
    else if (!data) {
        return <div>Loading...</div>
    }
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
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }            
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
                                buttonMessage={
                                    RustGenerateButtonText(item.status)
                                }   
                                passBackResponse={reRenderItems}/>
                    </>
                ))}
            </div>
            <CreateToDoItem passBackResponse={reRenderItems} />
        </div>
        </div>
    );

    // return (
    //     <div>
    //         {error ? (
    //             <div style={{ color: 'red' }}>Error: {error}</div>
    //         ) : data ? (
    //             <div>Data loaded: {JSON.stringify(data)}</div>
    //         ) : (
    //             <div>Loading...</div>
    //         )}
    //         <CreateToDoItem passBackResponse={reRenderItems} />
    //     </div>
    // );    
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
