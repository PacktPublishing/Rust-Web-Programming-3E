// File: frontend/src/api/utils.ts
import axios, {AxiosResponse} from "axios";


async function handleRequest<T, X>(
    promise: Promise<AxiosResponse<X>>, 
    expectedResponse: number) {
    let response: AxiosResponse<X>;
    try {
        response = await promise;
    } catch (error) {
        const status = error.response?.status || 500;
        const errorMessage = error.message || 'Network or unknown error'
        return {
            status: status,
            error: errorMessage
        };
    }
    if (response.status === expectedResponse) {
        return {
            status: response.status,
            data: response.data as X
        };
    } else {
        return {
            status: response.status,
            error: response.data as string
        };
    }    
}


export async function postCall<T, X>(
    url: string, body: T, 
    expectedResponse: number) {
    let response = axios.post<X | string>(
        url,
        body,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "jwt"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function getCall<X>(
    url: string, 
    expectedResponse: number) {
    let response =  axios.get<X | string>(
        url,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "jwt"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function deleteCall<X>(
    url: string, 
    expectedResponse: number) {
    let response = axios.delete<X | string>(
        url,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "jwt"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function putCall<T, X>(
    url: string, body: T, 
    expectedResponse: number) {
    let response = axios.put<X | string>(
        url,
        body,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': "jwt"
            },
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}
