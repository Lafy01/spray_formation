"use client"

import { columnsAllCongeEmploye } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { useAllMyEmployeConge } from "@/hooks/useConge"

export default function ListeCongePage() {
  // const { conge } = useAllConge();
  const { myEmployeConge } = useAllMyEmployeConge();
  console.log("Voici mon conge: ", myEmployeConge);  
  // const { users } = useAllUsers();
  // console.log("Voci les user conge: ", users);

  return (
    <div className=" h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Demande de Conge!</h2>
          <p className="text-muted-foreground">
            Tous les listes de demande de conges de votre employe.
          </p>
        </div>
      </div>

      <DataTable data={myEmployeConge} columns={columnsAllCongeEmploye} />
    </div>
  )
}