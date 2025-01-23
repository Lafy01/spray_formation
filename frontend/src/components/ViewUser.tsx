import { userType } from "@/schemas/schemaTable"
import { ViewEyeIcon } from "./icon/iconApp"
import { Button } from "./ui/button"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog"
import { Separator } from "./ui/separator"
import { Descriptions, DescriptionsProps } from "antd"


type viewUserProps = {
    user?: userType
}

export const ViewUser = ({ user }: viewUserProps) => {
    console.log("View User: ", user)
    console.log("Email chef: ", user?.departement.chef_dep?.email_empl)
    const userInfo: DescriptionsProps['items'] = [
        {
            key: '1',
            label: 'Nom',
            children: `${user?.nom_empl} ${user?.prenom_empl}`
        },
        {
            key: '2',
            label: 'Email',
            children: user?.email_empl
        },
        {
            key: '3',
            label: 'N. Matricule',
            children: user?.n_matricule
        },
        {
            key: '4',
            label: 'Role',
            children: user?.role
        },
        {
            key: '5',
            label: 'Status',
            children: user?.status
        }
    ]
    const DepartementInfo: DescriptionsProps['items'] = user?.role != 'apprenant' 
        ? undefined : 
        [
        {
            key: '1',
            label: 'Nom Departement',
            children: user?.departement?.nom_dep
        },
        {
            key: '2',
            label: 'Code Departement',
            children: user?.departement?.code_dep
        },
        {
            key: '3',
            label: 'Chef Departement',
            children: ((user?.departement?.chef_dep?.n_matricule == user?.n_matricule) ? "Me" : user?.departement?.chef_dep?.nom_empl + " " + user?.departement?.chef_dep?.prenom_empl)
        },
    ]

    const SoldeInfo: DescriptionsProps['items'] = [
        {
            key: '1',
            label: 'Solde Aquis',
            children: user?.solde.j_aqui_sld
        },
        {
            key: '2',
            label: 'Solde Pris',
            children: user?.solde.j_pris_sld
        },
        {
            key: '3',
            label: 'Solde Reste',
            children: user?.solde.j_reste_sld
        },
    ]
    return (
        <Dialog>
            <DialogTrigger asChild className="m-3">
                <span className="text-lg text-default-400 cursor-pointer active:opacity-50">
                    <ViewEyeIcon />
                </span>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[850px]">

                <DialogHeader>
                    <DialogTitle>Information {user?.role}</DialogTitle>
                    <DialogDescription>
                        Vous trouvez ici son information personnelle, departement, son solde.
                    </DialogDescription>
                </DialogHeader>

                <Separator className="" />
                <Descriptions title="User Info" layout="vertical" items={userInfo} />
                <Descriptions title="Formation Info" layout="vertical" items={DepartementInfo} />
                {/* <Descriptions title="Solde Info" layout="vertical" items={SoldeInfo} /> */}
            </DialogContent>
        </Dialog>
    )
}