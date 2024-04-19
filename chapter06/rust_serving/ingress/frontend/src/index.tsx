import React from 'react';
import ReactDOM from "react-dom/client";
// // @ts-ignore
// import styles from './index.module.css';
// import * as wasm from 'rust_interface';
// import init, { rust_add } from '../rust_interface/pkg/rust_interface';


const App = () => {

    return (
        <div>
            <h1>Hello World</h1>
        </div>
    );
};

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
