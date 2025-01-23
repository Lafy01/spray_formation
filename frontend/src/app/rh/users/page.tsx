"use client"
import { columnsUsers } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { UserNav } from "@/components/dataListe/user-nav";
import { useAllEmployes } from "@/hooks/useAllEmploye";

export default function UsersPage() {
  const { employes, error, isLoading } = useAllEmployes()

  return (
    <div className="hidden h-full flex-1 flex-col space-y-8 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Employees!</h2>
          <p className="text-muted-foreground">
            Here&apos;s a list of your tasks for this month!
          </p>
        </div>
      </div>
      <DataTable data={employes} columns={columnsUsers} />
    </div>
  );
}
