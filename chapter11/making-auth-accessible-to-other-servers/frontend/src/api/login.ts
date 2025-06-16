import axios from 'axios';
import { Url } from "./url";

export const login = async (
        email: string, 
        password: string
    ): Promise<string> => {
    const authToken = btoa(`${email}:${password}`);
    try {
        const response = await axios({
            method: 'get',
            url: new Url().login,
            headers: {
                'Authorization': `Basic ${authToken}`,
                'Content-Type': 'application/json'
            },
        });
        return response.data;
    }
    catch (error) {
        if (axios.isAxiosError(error)) {
            console.error('Login error:', error.response?.data);
        } else {
            console.error('Unexpected error:', error);
        }
        alert('Login failed. Please try again.');
        throw error;
    }
    
};
