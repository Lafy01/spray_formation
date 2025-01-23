"use client"

import { zodResolver } from "@hookform/resolvers/zod"
import { CalendarIcon } from "@radix-ui/react-icons"
import { format } from "date-fns"
import { useForm } from "react-hook-form"
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { Calendar } from "@/components/ui/calendar"
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form"
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import { toast } from "@/components/ui/use-toast"
import { Switch } from "./ui/switch"
import { Input } from "./ui/input"
import { InputDay } from "./ui/inputDays"
import { myCongeSchema, myCongeType } from "@/schemas/schemaTable"
import { useAddConge, useAllConge, useAllMyConge } from "@/hooks/useConge"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog"
import { Separator } from "./ui/separator"
import DatePicker, { DateRangeType } from "react-tailwindcss-datepicker"
import { useState } from "react"

export function AddConge() {
    const form = useForm<myCongeType>({
        resolver: zodResolver(myCongeSchema),
    })

    const { updateMyCongeData } = useAllMyConge();

    const { watch, setValue } = form;
    const debu_matin = watch("debut_matin");

    const [dateValue, setDateValue] = useState({
        startDate: null,
        endDate: null
    })

    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(0, 0, 0, 0);

    const handleDateChange = (newValue: any | null) => {
        setDateValue(newValue)
        if (newValue && newValue.startDate) {
            setValue("date_deb_cong", newValue.startDate);
        }
    }

    const { add_my_conge } = useAddConge();

    async function onSubmit(data: myCongeType) {
        const dateDebCong = new Date(data.date_deb_cong);

        const utcDate = new Date(Date.UTC(
            dateDebCong.getFullYear(), dateDebCong.getMonth(), dateDebCong.getDate(),
            !debu_matin ? 6 : 12
        ))

        console.log("Date en UTC: ", utcDate.toISOString());

        data.date_deb_cong = utcDate.toISOString();

        toast({
            title: "You submitted the following values:",
            description: (
                <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
                    <code className="text-white">{JSON.stringify(data, null, 2)}</code>
                </pre>
            ),
        })
        try {
            await add_my_conge(data);
            updateMyCongeData();
            form.reset();
        } catch (error) {
            console.error("Error lors de l'enregistrement");
            toast({
                title: "Error",
                description: "Une erreur est survenue lors de l'enregistrement.",
                variant: "destructive",
            })
        }
    }

    return (
        <Dialog>
            <DialogTrigger asChild className="m-3">
                <Button>Ajouter</Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[850px]">
                <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
                        <DialogHeader>
                            <DialogTitle>Ajouter un conge</DialogTitle>
                            <DialogDescription>
                                Ajouter un conge asdajpqqojpfsofq qwpdok dq
                            </DialogDescription>
                        </DialogHeader>

                        <Separator />

                        <FormField
                            control={form.control}
                            name="motif_cong"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Motif du conge</FormLabel>
                                    <FormControl>
                                        <Input {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="date_deb_cong"
                            render={({ field }) => (
                                <FormItem className="flex flex-col">
                                    <FormLabel>Debut du conge</FormLabel>
                                    <FormControl>
                                        <DatePicker placeholder="Selectionnez une date"
                                            asSingle={true}
                                            value={dateValue}
                                            onChange={handleDateChange}
                                            showShortcuts
                                            minDate={tomorrow}
                                        />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="debut_matin"
                            render={({ field }) => (
                                <FormItem className="flex flex-col">
                                    <div className="flex items-center space-x-2">
                                        <FormControl>
                                            <Switch
                                                checked={field.value}
                                                onCheckedChange={field.onChange}
                                            />
                                        </FormControl>
                                        <FormLabel>Apres midi</FormLabel>
                                    </div>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <FormField
                            control={form.control}
                            name="nb_jour_cong"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Nombre de Jour</FormLabel>
                                    <FormControl>
                                        <InputDay {...field} />
                                    </FormControl>
                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <DialogFooter>
                            <Button type="submit">Submit</Button>
                        </DialogFooter>
                    </form>
                </Form>
            </DialogContent>
        </Dialog>
    )
}


// <Popover>
//     <PopoverTrigger asChild>
//         <FormControl>
//             <Button
//                 variant={"outline"}
//                 className={cn(
//                     "w-[240px] pl-3 text-left font-normal",
//                     !field.value && "text-muted-foreground"
//                 )}
//             >
//                 {field.value ? (
//                     format(new Date(field.value), "PPP")
//                 ) : (
//                     <span>Prenez une date</span>
//                 )}
//                 <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
//             </Button>
//         </FormControl>
//     </PopoverTrigger>
//     <PopoverContent className="w-auto p-0 z-[1000]" align="start">
//         <Calendar
//             mode="single"
//             selected={field.value}
//             onSelect={field.onChange}
//             disabled={(date) =>
//                 date > new Date() || date < new Date("1900-01-01")
//             }
//             initialFocus
//         />
//     </PopoverContent>
// </Popover>