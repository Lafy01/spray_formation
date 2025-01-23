"use client"
import { useProtectedRoute } from "@/hooks/useProtectedRoute";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

export default function Home() {
  const { loading, authentificated, role} = useProtectedRoute();
  const router = useRouter()

  if (loading) {
    return <div>Chargement...</div>;
  }

  if (!authentificated) {
    return toast.error("Vous devez être authentifié pour accéder à cette page.");
  }

  router.push(`/${role}/dashboard`);
  return null;
}