import { User } from "./Employe";

export type AuthState = {
    user: User | null;
    token: string | null;

    setUser: (user: User ) => void;
    setToken: (token: string, role: string) => void;
    logout: () => void;
}