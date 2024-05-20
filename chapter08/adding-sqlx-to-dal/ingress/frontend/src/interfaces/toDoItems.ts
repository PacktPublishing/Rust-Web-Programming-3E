

export enum TaskStatus {
    PENDING = 'PENDING',
    DONE = 'DONE'
}


export interface ToDoItem {
    title: string;
    status: TaskStatus;
}


export interface ToDoItems {
    pending: ToDoItem[];
    done: ToDoItem[];
}
