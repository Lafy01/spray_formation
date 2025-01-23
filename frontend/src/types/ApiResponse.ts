export type ApiResponse<T> = {
    status_code: number;
    success: boolean;
    message: string;
    data: T;
}