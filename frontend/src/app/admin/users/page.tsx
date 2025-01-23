"use client"
import { AddUser } from "@/components/AddUser";
import { columnsUsers } from "@/components/dataListe/columns";
import { DataTable } from "@/components/dataListe/data-table";
import { useAllUsers } from "@/hooks/useAllEmploye";

export default function UsersPage() {
  const { users } = useAllUsers();

  console.log("Voici les users: ", users);

  return (
  <div className=" h-full flex-1 flex-col space-y-1 p-8 md:flex">
      <div className="flex items-center justify-between space-y-2">
        <div>
          <h2 className="text-2xl font-bold tracking-tight">Liste des utilisateur</h2>
          <p className="text-muted-foreground">
            Voici la liste de tous les utilisateur.
          </p>
        </div>
        <div className="flex items-center space-x-2 ">
          <AddUser />
        </div>
      </div>
      <DataTable data={users} columns={columnsUsers} />
    </div>
  );
}
