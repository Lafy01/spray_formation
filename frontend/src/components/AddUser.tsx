import { Button } from "./ui/button"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "./ui/form"
import { Input } from "./ui/input"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { toast } from "./ui/use-toast"
import { userRegister, userRegisterType } from "@/schemas/schemaTable"
import { useAllDepartement } from "@/hooks/useDepartement"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "./ui/select"
import { Departement } from '@/types/Employe';
import { Separator } from "./ui/separator"
import { useRegister } from "@/hooks/useAuth"
import { toast as toastSonner } from 'sonner';
import { EditIcon } from "./icon/iconApp"
import { useAllUsers } from "@/hooks/useAllEmploye"
import { PasswordInput } from "./ui/input-password"

type addUserProps = {
    user?: userRegisterType;
}

export const AddUser = ({ user }: addUserProps) => {

    const { departements } = useAllDepartement();
    const { updateUserData } = useAllUsers();
    const { register } = useRegister();

    const form = useForm<userRegisterType>({
        resolver: zodResolver(userRegister),
        defaultValues: {
            n_matricule: user?.n_matricule || "",
            id_dep: "",
            nom_empl: user?.nom_empl || "",
            prenom_empl: user?.prenom_empl || "",
            email_empl: user?.email_empl || "",
            passw_empl: user?.passw_empl || "",
            role: user?.role || "employe",
        },
    })

    async function onSubmit(data: userRegisterType) {
        toast({
            title: "You submitted the following values:",
            description: (
                <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
                    <code className="text-white">{JSON.stringify(data, null, 2)}</code>
                </pre>
            ),
        })
        try {
            if (user) {
                toastSonner.success("Modification reussie");
            } else {
                form.reset();
                await register(data);
                updateUserData();
            }
            form.reset();
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
            <DialogTrigger asChild className={ user ? "m-3" : "m-10"}>
                { user ? 
                <EditIcon />: <Button>Ajouter</Button> }
            </DialogTrigger>
            <DialogContent className="sm:max-w-[850px]">
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">

                        <DialogHeader>
                            <DialogTitle>{ user ? "Modification utilisateur" : "Ajout utilisateur"}</DialogTitle>
                            <DialogDescription>
                                Ajouter tous vos information
                            </DialogDescription>
                        </DialogHeader>

                        <Separator className="" />

                        <FormField
                            control={form.control}
                            name="id_dep"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Formation</FormLabel>
                                    <Select onValueChange={field.onChange} defaultValue={field.value} >
                                        <FormControl>
                                            <SelectTrigger>
                                                <SelectValue placeholder="Choisisez votre formation" />
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
                                            <Input placeholder="numéro matricule" {...field} />
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
                                        <FormLabel>Rôle</FormLabel>
                                        <Select onValueChange={field.onChange} defaultValue={field.value} >
                                            <FormControl>
                                                <SelectTrigger>
                                                    <SelectValue placeholder="Choisisez votre Role" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectItem value="apprenant">apprenant</SelectItem>
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
                        <FormField
                            control={form.control}
                            name="passw_empl"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Mot de Passe</FormLabel>
                                    <FormControl>
                                        <PasswordInput placeholder="Entrer votre mot de passe" {...field} />
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