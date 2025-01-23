import { useAuthStore } from "@/stores/AuthStore";
import axios from "axios";


export const apiUrl = process.env.NEXT_PUBLIC_API_BASE_URL;

const axiosInstance = axios.create({
    baseURL: `${apiUrl}`,
    headers: {
        "Content-Type": "application/json",
    }
})

axiosInstance.interceptors.request.use(
    (config) => {
        const { token } = useAuthStore.getState();
        console.log("token avant setting: ", token)
        if (token) {
            config.headers.Authorization = `Bearer ${token}`;
        }
        console.log(config.headers.Authorization);
        return config;
    },
    (error) => Promise.reject(error)
);

axiosInstance.interceptors.response.use(
    response => response,

    error => {
        if (error.response && error.response.status === 401) {
            // Logout the user if token is expired
            useAuthStore.getState().logout();
            // Redirect to login page
            window.location.href = '/authentification';
        }
        return Promise.reject(error);
    }
)

export default axiosInstance;