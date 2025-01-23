import axiosInstance from "./axiosInstance"


export const fetcher = async (url: string) => {
    try {
        const response = await axiosInstance.get(url);
        return response.data;
    } catch (error: any) {
        if (error.response) {
            console.error("Error response: ", error.response);
            throw new Error(error.response.data.message || "An error occurred");
        } else if (error.request) {
            console.error("No response received: ", error.request);
            throw new Error("No response received");
        } else {
            console.error("Error: ", error.message);
            throw new Error(error.message);
        }
    }
}