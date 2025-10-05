

export class Url {

    baseUrl: string;
    create: string;
    getAll: string;
    update: string;
    login: string;

    constructor() {
        this.baseUrl = Url.getBaseUrl();
        this.create = `${this.baseUrl}api/v1/create`;
        this.getAll = `${this.baseUrl}api/v1/get/all`;
        this.update = `${this.baseUrl}api/v1/update`;
        this.login = `${this.baseUrl}api/v1/auth/login`;
    }

    static getBaseUrl(): string {
        let url = window.location.href;
        if (url.includes("http://localhost:3000/")) {
            return "http://0.0.0.0:8001/";
        }
        return window.location.href;
    }

    deleteUrl(name: string): string {
        return `${this.baseUrl}api/v1/delete/${name}`;
    }
}
