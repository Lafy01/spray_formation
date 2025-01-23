"use client"
import { AddDepartement } from "@/components/AddDepartement";
import { columnsDepartements } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { UserNav } from "@/components/dataListe/user-nav";
import { useAllDepartement } from "@/hooks/useDepartement";

export default function DepartementPage() {
  const { departements, error, isLoading } = useAllDepartement();

  console.log("Voici les departement: ", departements);
  return (
    <div className="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Formation!</h2>
          <p className="text-muted-foreground">
Voici la liste de tous les formation.
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <AddDepartement />
        </div>
      </div>
      <DataTable data={departements} columns={columnsDepartements} />
    </div>
  );
}
