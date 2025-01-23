import { User } from "@/types/Employe";
import { AuthState } from "@/types/authentification";
import { create } from "zustand";


export const useAuthStore = create<AuthState>()(
    (set) => ({
        user: typeof window !== 'undefined' ? JSON.parse(localStorage.getItem('user') || 'null') : null,
        token: typeof window !== 'undefined' ? localStorage.getItem('token') : null,
        setToken: (token: string, role: string) => {
            localStorage.setItem('token', token);
            localStorage.setItem('role', role);
            
            set({ token })
        },
        setUser: (user: User) => {
            localStorage.setItem('user', JSON.stringify(user));
            set({ user });
        },
        logout: () => {
            localStorage.removeItem('token');
            localStorage.removeItem('role');
            localStorage.removeItem('user');
            set({ user: null, token: null })
        }
    })
)