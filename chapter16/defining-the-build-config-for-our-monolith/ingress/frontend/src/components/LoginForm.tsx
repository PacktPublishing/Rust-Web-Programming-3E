import React from 'react';
import '../Login.css';
import { login } from '../api/login';

interface LoginFormProps {
    setToken: (token: string) => void;
}

export const LoginForm: React.FC<LoginFormProps> = ({ setToken }) => {
    const [email, setEmail] = React.useState<string>("");
    const [password, setPassword] = React.useState<string>("");

    const submitLogin = () => {
        login(email, password).then(
            (response) => {
                setToken(response);
            }
        ).catch((error) => {
            console.error(error);
        });
    };

    const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(e.target.value);
    };

    const handleUsernameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setEmail(e.target.value);
    };

    return (
        <div className="login">
            <h1 className="login-title">Login</h1>
            <input
                type="text"
                className="login-input"
                placeholder="Email"
                autoFocus
                onChange={handleUsernameChange}
                value={email}
            />
            <input
                type="password"
                className="login-input"
                placeholder="Password"
                onChange={handlePasswordChange}
                value={password}
            />
            <button className="login-button"
                 id="login-button"
                 onClick={submitLogin}>Lets Go</button>
        </div>
    );
};
