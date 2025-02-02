// File: frontend/src/api/url.ts
export class Url {

    baseUrl: string;
    create: string;
    getAll: string;
    update: string;

    constructor() {
        this.baseUrl = Url.getBaseUrl();
        this.create = `${this.baseUrl}api/v1/create`;
        this.getAll = `${this.baseUrl}api/v1/get/all`;
        this.update = `${this.baseUrl}api/v1/update`;
    }
    
    static getBaseUrl(): string {
        let url = window.location.href;
        if (url.includes("http://localhost:3000/")) {
            return "http://0.0.0.0:8001/";
        }
        return url;
    }
    
    deleteUrl(name: string): string {
        return `${this.baseUrl}api/v1/delete/${name}`;
    }
}
