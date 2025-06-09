import { postCall } from '../utils';


describe('postCall test', () => {

    test('test1 should log test1', async () => {
        const url = 'https://jsonplaceholder.typicode.com/posts'; // Example test server
        const body = {
            title: 'foo',
            body: 'bar',
            userId: 1,
        };
        const expectedResponse = 200; // Expected status code for a successful POST

        // Await the async postCall function
        const result = await postCall<typeof body, { id: number }>(
            url,
            body,
            expectedResponse
        );
        console.log(result);
        // Assertions for the response
        // expect(result).toHaveProperty('id');
        // expect(typeof result.id).toBe('number');
    });

});
