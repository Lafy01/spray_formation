
import { userType } from "@/schemas/schemaTable"
import { ViewEyeIcon } from "./icon/iconApp"
import { Button } from "./ui/button"
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog"
import { Separator } from "./ui/separator"
import { Badge, Descriptions, DescriptionsProps } from "antd"


type viewUserProps = {
    user?: userType
}

export const MyProfile = ({ user }: viewUserProps) => {
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
            label: 'Rôle',
            children: user?.role
        },
        {
            key: '5',
            label: 'Status',
            children: <Badge status="processing" text={user?.status} />
        }
    ]
    const DepartementInfo: DescriptionsProps['items'] = [
        {
            key: '1',
            label: 'Nom département',
            children: user?.departement?.nom_dep
        },
        {
            key: '2',
            label: 'Code département',
            children: user?.departement?.code_dep
        },
        {
            key: '3',
            label: 'Chef département',
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
        <>
            <Separator className="" />
            <Descriptions title="Information utilisateur" layout="vertical" items={userInfo} bordered/>
        </>
    )
}