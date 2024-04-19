import React, { useState } from 'react';
import ReactDOM from "react-dom/client";
import getAll from './api/get';
import {ToDoItems} from "./interfaces/toDoItems";
import { ToDoItem } from "./components/ToDoItem";

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

    function logOut() {

    }

    // on load
    React.useEffect(() => {
        const fetchData = async () => {
            const response = await getAll<ToDoItems>(window.location.href);
            setData(response.data);
        }
        fetchData();
    }, []);

    if (!data) {
        return <div>Loading...</div>;
    }
    else {
        return (
            <div>
                <h1>Pending Items</h1>
                <div>
                    {data.pending.map((item, index) => (
                        <><ToDoItem key={item.title + item.status}
                                    title={item.title}
                                    status={item.status}
                                    passBackResponse={reRenderItems}
                                    logout={logOut}/>
                        </>
                    ))}
                </div>
                <h1>Done Items</h1>
                <div>
                    {data.done.map((item, index) => (
                        <><ToDoItem key={item.title + item.status}
                                    title={item.title}
                                    status={item.status}
                                    passBackResponse={reRenderItems}
                                    logout={logOut}/>
                        </>
                    ))}
                </div>
                <CreateToDoItem passBackResponse={(response) => setData(response.data)} />
            </div>
        );
    }

    // return (
    //     <div>
    //         {/*<h1>Pending Items</h1>*/}
    //         {/*{this.state.pending_items}*/}
    //         {/*<h1>Done Items</h1>*/}
    //         {/*{this.state.done_items}*/}
    //         {data ? <div>Data loaded: {JSON.stringify(data)}</div> : <div>Loading...</div>}
    //         <CreateToDoItem passBackResponse={(response) => setData(response.data)} />
    //     </div>
    // );
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
