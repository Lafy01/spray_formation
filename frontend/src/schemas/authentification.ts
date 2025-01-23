import { z } from "zod";

export const loginSchema = z.object({
    email: z.string().min(1, "Verifier votre email"),
    password: z.string().min(6, "Verifier votre mot de passe"),
});