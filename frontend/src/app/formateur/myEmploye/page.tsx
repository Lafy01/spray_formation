"use client"
import { columnsUsers } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { UserNav } from "@/components/dataListe/user-nav";
import { useAllMyEmployes } from "@/hooks/useAllEmploye";

export default function MyEmployePage() {
  const { myEmployes, error, isLoading } = useAllMyEmployes()

  return (
    <div className="h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Mes Employe</h2>
          <p className="text-muted-foreground">
            Here&apos;s a list of your tasks for this month!
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <UserNav />
        </div>
      </div>
      <DataTable data={myEmployes} columns={columnsUsers} />
    </div>
  );
}