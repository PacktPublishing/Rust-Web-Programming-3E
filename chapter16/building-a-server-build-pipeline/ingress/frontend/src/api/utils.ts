import axios, {AxiosResponse} from "axios";


async function handleRequest<T, X>(promise: Promise<AxiosResponse<X>>, expectedResponse: number) {
    let response: AxiosResponse<X>;
    try {
        response = await promise;
    } catch (error) {
        return {
            status: 500,
            error: 'Network or other error occurred'
        };
    }
    if (response.status === expectedResponse) {
        // Successful response
        return {
            status: response.status,
            data: response.data as X
        };
    } else {
        // Other response codes with a string message
        return {
            status: response.status,
            error: response.data as string
        };
    }
}


export async function postCall<T, X>(url: string, body: T, expectedResponse: number) {
    let response = axios.post<X | string>(
        url,
        body,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': localStorage.getItem('token')
            },
            // Resolve the promise for all HTTP status codes
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function deleteCall<X>(url: string, expectedResponse: number) {
    let response = axios.delete<X | string>(
        url,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': localStorage.getItem('token')
            },
            // Resolve the promise for all HTTP status codes
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function putCall<T, X>(url: string, body: T, expectedResponse: number) {
    let response = axios.put<X | string>(
        url,
        body,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': localStorage.getItem('token')
            },
            // Resolve the promise for all HTTP status codes
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}


export async function getCall<X>(url: string, expectedResponse: number) {
    let response =  axios.get<X | string>(
        url,
        {
            headers: {
                'Content-Type': 'application/json',
                'token': localStorage.getItem('token')
            },
            // Resolve the promise for all HTTP status codes
            validateStatus: () => true
        });
    return handleRequest(response, expectedResponse);
}