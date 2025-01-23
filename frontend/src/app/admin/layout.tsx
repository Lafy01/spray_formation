"use client"
import { useProtectedRoute } from "@/hooks/useProtectedRoute";
import { toast } from "sonner";

export default function EmployeLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const { loading, authentificated } = useProtectedRoute();

    if (loading) {
        return <div>Chargement...</div>;
    }

    if (!authentificated) {
        return toast.error("Vous devez être authentifié pour accéder à cette page.");
    }
    return (
        <>
            {children}
        </>
    );
}
