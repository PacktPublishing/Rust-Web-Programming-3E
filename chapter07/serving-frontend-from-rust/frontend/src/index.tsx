import React from 'react';
import ReactDOM from "react-dom/client";

const App = () => {
    return (
        <div>
            <h1>Hello World</h1>
        </div>
    );
};
const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);
