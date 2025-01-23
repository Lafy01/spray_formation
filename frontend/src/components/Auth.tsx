"use client"
// import { Button } from "@/components/ui/button"
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { loginSchema } from "@/schemas/authentification"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "./ui/form"
import { PasswordInput } from "./ui/input-password"
import { Email, IdCard, Lock } from "./icon/iconApp"
import { toast } from "sonner"
import { useRouter } from "next/navigation"
import { useLogin } from "@/hooks/useAuth"
import { Button } from "antd"

export function LoginForm() {
    const goTo = useRouter();
    const form = useForm<z.infer<typeof loginSchema>>({
        resolver: zodResolver(loginSchema),
        defaultValues: {
            email: "",
            password: "",
        },
    })

    const { login } = useLogin();

    const onSubmit = async (values: z.infer<typeof loginSchema>) => {
        try {
            const res = await login(values.email, values.password);
            console.log("Response login: ", res);
        } catch (error) {
            console.log(error);
        }
    }

    const onFinish = async () => {

    }

    return (

        <Card className="w-full max-w-sm">
            <CardHeader>
                <CardTitle className="text-2xl text-center">Bienvenue à Spray Info</CardTitle>
                <CardDescription>
                    Entrez votre numero matricule ci-dessous pour vous connecter à votre compte.
                </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-4">
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)}
                        className="space-y-8"
                    >
                        <FormField control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>N.matricule</FormLabel>
                                    <FormControl>
                                        <Input placeholder="Numero matricule *" {...field} suffix={<IdCard />} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField control={form.control}
                            name="password"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Mot de passe</FormLabel>
                                    <FormControl>
                                        <PasswordInput placeholder="Mot de passe" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <Button type="primary" htmlType="submit" className="w-full">Submit</Button>
                    </form>
                </Form>
            </CardContent>
        </Card>
    )
}
