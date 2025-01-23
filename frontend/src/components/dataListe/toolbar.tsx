"use client"

import { Cross2Icon } from "@radix-ui/react-icons"
import { Row, Table } from "@tanstack/react-table"

// import { DataTableViewOptions } from "@/app/(app)/examples/tasks/components/data-table-view-options"

import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { priorities, statusCong, statuses } from "@/configs/site"
import { DataTableFacetedFilter, DataTableFacetedFilterAllConge } from "./faceted-filter"
import { DataTableViewOptions } from "./view-option"

interface DataTableToolbarProps<TData> {
    table: Table<TData>
}

export function DataTableToolbarUsers<TData>({
    table,
}: DataTableToolbarProps<TData>) {
    const isFiltered = table.getState().columnFilters.length > 0

    // const filterByName = (value: string) => {
    //     table
    //         .getColumn("nom_empl")
    //         ?.setFilterValue((row: Row<TData>) => {
    //             const nomEmpl = row.getValue("nom_empl").toLowerCase();
    //             const prenomEmpl = row.getValue("prenom_empl").toLowerCase();
    //             return (
    //                 nomEmpl.includes(value.toLowerCase()) ||
    //                 prenomEmpl.includes(value.toLowerCase())
    //             );
    //         });
    // };

    return (
        <div className="flex items-center justify-between">
            <div className="flex flex-1 items-center space-x-2">
                <Input
                    placeholder="Filtré par Nom..."
                    value={(table.getColumn("nom_empl")?.getFilterValue() as string) ?? ""}
                    onChange={(event) => {
                        table.getColumn("nom_empl")?.setFilterValue(event.target.value)
                    }}
                    className="h-8 w-[150px] lg:w-[250px]"
                />
                
                {isFiltered && (
                    <Button
                        variant="ghost"
                        onClick={() => table.resetColumnFilters()}
                        className="h-8 px-2 lg:px-3"
                    >
                        Réinitialiser
                        <Cross2Icon className="ml-2 h-4 w-4" />
                    </Button>
                )}
            </div>
            <DataTableViewOptions table={table} />
        </div>
    )
}

export function DataTableToolbarConge<TData>({
    table,
}: DataTableToolbarProps<TData>) {
    const isFiltered = table.getState().columnFilters.length > 0

    // const filterByName = (value: string) => {
    //     table
    //         .getColumn("nom_empl")
    //         ?.setFilterValue((row: Row<TData>) => {
    //             const nomEmpl = row.getValue("nom_empl").toLowerCase();
    //             const prenomEmpl = row.getValue("prenom_empl").toLowerCase();
    //             return (
    //                 nomEmpl.includes(value.toLowerCase()) ||
    //                 prenomEmpl.includes(value.toLowerCase())
    //             );
    //         });
    // };

    return (
        <div className="flex items-center justify-between">
            <div className="flex flex-1 items-center space-x-2">
                <Input
                    placeholder="Filtré par motif..."
                    value={(table.getColumn("motif_cong")?.getFilterValue() as string) ?? ""}
                    onChange={(event) => {
                        table.getColumn("motif_cong")?.setFilterValue(event.target.value)
                    }}
                    className="h-8 w-[150px] lg:w-[250px]"
                />
                
                {isFiltered && (
                    <Button
                        variant="ghost"
                        onClick={() => table.resetColumnFilters()}
                        className="h-8 px-2 lg:px-3"
                    >
                        Réinitialiser
                        <Cross2Icon className="ml-2 h-4 w-4" />
                    </Button>
                )}
            </div>
            <DataTableViewOptions table={table} />
        </div>
    )
}
export function DataTableToolbarDepartement<TData>({
    table,
}: DataTableToolbarProps<TData>) {
    const isFiltered = table.getState().columnFilters.length > 0

    // const filterByName = (value: string) => {
    //     table
    //         .getColumn("nom_empl")
    //         ?.setFilterValue((row: Row<TData>) => {
    //             const nomEmpl = row.getValue("nom_empl").toLowerCase();
    //             const prenomEmpl = row.getValue("prenom_empl").toLowerCase();
    //             return (
    //                 nomEmpl.includes(value.toLowerCase()) ||
    //                 prenomEmpl.includes(value.toLowerCase())
    //             );
    //         });
    // };

    return (
        <div className="flex items-center justify-between">
            <div className="flex flex-1 items-center space-x-2">
                <Input
                    placeholder="Filtré par nom de la formation ..."
                    value={(table.getColumn("nom_dep")?.getFilterValue() as string) ?? ""}
                    onChange={(event) => {
                        table.getColumn("nom_dep")?.setFilterValue(event.target.value)
                    }}
                    className="h-8 w-[150px] lg:w-[250px]"
                />
                
                {isFiltered && (
                    <Button
                        variant="ghost"
                        onClick={() => table.resetColumnFilters()}
                        className="h-8 px-2 lg:px-3"
                    >
                        Réinitialiser
                        <Cross2Icon className="ml-2 h-4 w-4" />
                    </Button>
                )}
            </div>
            <DataTableViewOptions table={table} />
        </div>
    )
}


export function DataTableToolbarRh<TData>({
    table,
}: DataTableToolbarProps<TData>) {
    const isFiltered = table.getState().columnFilters.length > 0

    // const filterByName = (value: string) => {
    //     table
    //         .getColumn("nom_empl")
    //         ?.setFilterValue((row: Row<TData>) => {
    //             const nomEmpl = row.getValue("nom_empl").toLowerCase();
    //             const prenomEmpl = row.getValue("prenom_empl").toLowerCase();
    //             return (
    //                 nomEmpl.includes(value.toLowerCase()) ||
    //                 prenomEmpl.includes(value.toLowerCase())
    //             );
    //         });
    // };

    return (
        <div className="flex items-center justify-between">
            <div className="flex flex-1 items-center space-x-2">
                <Input
                    placeholder="Filtré par nom..."
                    value={(table.getColumn("nom_empl")?.getFilterValue() as string) ?? ""}
                    onChange={(event) => {
                        table.getColumn("nom_empl")?.setFilterValue(event.target.value)
                    }}
                    className="h-8 w-[150px] lg:w-[250px]"
                />
                {table.getColumn("status_cong") && (
                    <DataTableFacetedFilterAllConge
                        column={table.getColumn("status_cong")}
                        title="Status"
                        options={statusCong}
                    />
                )}
                
                {isFiltered && (
                    <Button
                        variant="ghost"
                        onClick={() => table.resetColumnFilters()}
                        className="h-8 px-2 lg:px-3"
                    >
                        Réinitialiser
                        <Cross2Icon className="ml-2 h-4 w-4" />
                    </Button>
                )}
            </div>
            <DataTableViewOptions table={table} />
        </div>
    )
}