import { Button } from "./ui/button"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "./ui/form"
import { Input } from "./ui/input"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { toast } from "./ui/use-toast"
import { userRegister, userRegisterType, userSchema, userType, userWithoutDepartement, userWithoutDepartementType } from "@/schemas/schemaTable"
import { useAllDepartement } from "@/hooks/useDepartement"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./ui/select"
import { Departement } from '@/types/Employe';
import { Separator } from "./ui/separator"
import { useRegister } from "@/hooks/useAuth"
import { toast as toastSonner } from 'sonner';
import { EditIcon } from "./icon/iconApp"
import { useUpdateUser } from "@/hooks/useEmploye"
import { useAllUsers } from "@/hooks/useAllEmploye"

type thisUserProps = {
    user: userType;
}

export const EditUser = ({ user }: thisUserProps) => {

    const { departements } = useAllDepartement();
    const { updateUser } = useUpdateUser();
    const { updateUserData } = useAllUsers();
    console.log("Departement:")

    const form = useForm<userWithoutDepartementType>({
        resolver: zodResolver(userWithoutDepartement),
        defaultValues: {
            n_matricule: user?.n_matricule || "",
            id_dep: user?.departement?.nom_dep || "",
            nom_empl: user?.nom_empl || "",
            prenom_empl: user?.prenom_empl || "",
            email_empl: user?.email_empl || "",
            role: user?.role || "",
        },
    })

    async function onSubmit(data: userWithoutDepartementType) {
        toast({
            title: "You submitted the following values:",
            description: (
                <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
                    <code className="text-white">{JSON.stringify(data, null, 2)}</code>
                </pre>
            ),
        })
        try {
            await updateUser(user.id_empl, data);
            form.reset();
            updateUserData();
        } catch (error) {
            console.error("Erreur lors de l'enregistrement")
            toast({
                title: "Erreur",
                description: "Une erreur est survenue lors de l'enregistrement.",
                variant: "destructive",
            })
        }
    }

    return (
        <Dialog>
            <DialogTrigger asChild className="m-3">
                <EditIcon />
            </DialogTrigger>
            <DialogContent className="sm:max-w-[850px]">
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">

                        <DialogHeader>
                            <DialogTitle>Edit {user.nom_empl} {user.prenom_empl}</DialogTitle>
                            <DialogDescription>
                                Make changes to your profile here. Click save when you&apos;re done.
                            </DialogDescription>
                        </DialogHeader>

                        <Separator className="" />

                        <FormField
                            control={form.control}
                            name="id_dep"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Departement</FormLabel>
                                    <Select onValueChange={field.onChange} defaultValue={field.value} >
                                        <FormControl>
                                            <SelectTrigger>
                                                <SelectValue placeholder="Choisisez votre departement" />
                                            </SelectTrigger>
                                        </FormControl>
                                        <SelectContent>
                                            {
                                                departements.map((dep: Departement) => (
                                                    <SelectItem key={dep.id_dep} value={dep.id_dep}>
                                                        {dep.nom_dep}
                                                    </SelectItem>
                                                ))
                                            }
                                        </SelectContent>
                                    </Select>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <div className="grid grid-cols-2 gap-4">
                            <FormField
                                control={form.control}
                                name="n_matricule"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>N. Matricule</FormLabel>
                                        <FormControl>
                                            <Input placeholder="numero matricule" {...field} />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="role"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Role</FormLabel>
                                        <Select onValueChange={field.onChange} defaultValue={field.value} >
                                            <FormControl>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Choisisez votre Role" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectItem value="apprenant">Employé</SelectItem>
                                                <SelectItem value="formateur">Formateur</SelectItem>
                                                <SelectItem value="admin">Administrateur</SelectItem>
                                            </SelectContent>
                                        </Select>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>

                        <div className="grid grid-cols-2 gap-4">
                            <FormField
                                control={form.control}
                                name="nom_empl"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Nom</FormLabel>
                                        <FormControl>
                                            <Input placeholder="Votre nom" {...field} />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                            <FormField
                                control={form.control}
                                name="prenom_empl"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormLabel>Prenom</FormLabel>
                                        <FormControl>
                                            <Input placeholder="Votre prenom" {...field} />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />

                        </div>

                        <FormField
                            control={form.control}
                            name="email_empl"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Email</FormLabel>
                                    <FormControl>
                                        <Input placeholder="Votre email" type="email" {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <DialogFooter>
                            <Button type="submit">Ajouter</Button>
                        </DialogFooter>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    )
}