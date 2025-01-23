"use client"
import { useRouter } from "next/navigation";

export default function EmployePage() {
  const router = useRouter();
  router.push('/manager/dashboard')
  return null;
}
