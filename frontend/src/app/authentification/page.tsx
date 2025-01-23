"use client"
import { LoginForm } from "@/components/Auth";
import { useAuthStore } from "@/stores/AuthStore";

export default function AuthPage() {
    const {user} = useAuthStore();
    console.log("Me: ");
    console.log(user)
    return (
        <LoginForm />
    )
}