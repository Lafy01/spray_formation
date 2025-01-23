"use client"
import { useAuthStore } from "@/stores/AuthStore";
import { usePathname, useRouter } from "next/navigation";
import { useEffect, useState } from "react";


export function useProtectedRoute() {
    const [loading, setLoading] = useState(true);
    const [authentificated, setAuthentificated] = useState(false);
    const pathname = usePathname();

    const { user } = useAuthStore();
    
    const router = useRouter();

    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    const role = typeof window !== 'undefined' ? localStorage.getItem('role') : null;
    useEffect(() => {
        const isAuthenticated = !!token && !!role && !!user;

        const isAuthorizedPath = pathname.includes(`/${role}`) || pathname === '/';

        if (!isAuthenticated || !isAuthorizedPath) {
            router.push('/authentification')
        } else {
            setAuthentificated(true);
            setLoading(false)
        }
    }, [token, role, user, router, pathname]);

    return {loading, authentificated, role};
}