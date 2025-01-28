"use client"

import { AddConge } from "@/components/AddConge"
import { columnsMyConge } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { useAllMyConge } from "@/hooks/useConge"

export default function MonCongePage() {
  // const { conge } = useAllConge();
  const { myConge } = useAllMyConge();
  console.log("Voici mon conge: ", myConge);  
  // const { users } = useAllUsers();
  // console.log("Voci les user conge: ", users);

  return (
    <div className=" h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Welcome back!</h2>
          <p className="text-muted-foreground">
            Here&apos;s a list of your tasks for this month!
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <AddConge />
        </div>
      </div>

      <DataTable data={myConge} columns={columnsMyConge} />
    </div>
  )
}