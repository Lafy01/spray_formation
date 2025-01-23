import {
    ArrowDownIcon,
    ArrowRightIcon,
    ArrowUpIcon,
    CheckCircledIcon,
    CircleIcon,
    CrossCircledIcon,
    QuestionMarkCircledIcon,
    StopwatchIcon,
} from "@radix-ui/react-icons";

export const labels = [
    {
        value: "bug",
        label: "Bug",
    },
    {
        value: "feature",
        label: "Feature",
    },
    {
        value: "documentation",
        label: "Documentation",
    },
];

export const statuses = [
    {
        value: "backlog",
        label: "Backlog",
        icon: QuestionMarkCircledIcon,
    },
    {
        value: "todo",
        label: "Todo",
        icon: CircleIcon,
    },
    {
        value: "in progress",
        label: "In Progress",
        icon: StopwatchIcon,
    },
    {
        value: "done",
        label: "Done",
        icon: CheckCircledIcon,
    },
    {
        value: "canceled",
        label: "Canceled",
        icon: CrossCircledIcon,
    },
];

export const statusCong = [
    {
        value: "En attente",
        label: "En attente RH",
    },
    {
        value: "En attente superieur",
        label: "En attente superieur",
    },
    {
        value: "Aprouver",
        label: "Aprouver",
    },
    {
        value: "Refuser",
        label: "Refuser",
    },
];

export const priorities = [
    {
        label: "Low",
        value: "low",
        icon: ArrowDownIcon,
    },
    {
        label: "Medium",
        value: "medium",
        icon: ArrowRightIcon,
    },
    {
        label: "High",
        value: "high",
        icon: ArrowUpIcon,
    },
];

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

export type SiteConfig = typeof siteConfig;

export const siteConfig = {
    name: "Gestion et suivi de conge",
    description:
        "Faire une application de gestion et suivi de conge des employes au seins de spat",

    navRH: [
        {
            label: "Tableau de bord",
            href: "/rh/dashboard",
        },
        {
            label: "Utilisateurs",
            href: "/rh/users",
        },
        {
            label: "Liste_Congé",
            href: "/rh/listeConge",
        },
        {
            label: "Planification",
            href: "/rh/planing",
        },
    ],

    navEmploye: [
        {
            label: "Tableau de bord",
            href: "/employe/dashboard",
        },
        {
            label: "Mon Conge",
            href: "/employe/monConge",
        },
        {
            label: "Calendrier",
            href: "/employe/calendrier",
        },
    ],

    navAdmin: [
        {
            label: "Table de bord",
            href: "/admin/dashboard",
        },
        {
            label: "Utilisateurs",
            href: "/admin/users",
        },
        {
            label: "Département",
            href: "/admin/departement",
        },
    ],

    navChefDTP: [
        {
            label: "Tableau de bord",
            href: "/chefdtp/dashboard",
        },
        {
            label: "Mon Conge",
            href: "/chefdtp/monConge",
        },

        {
            label: "Mes employés",
            href: "/chefdtp/myEmploye",
        },
        {
            label: "Demande de Conge",
            href: "/chefdtp/listeConge",
        },
        {
            label: "Mon Calendrier",
            href: "/chefdtp/monCalendrier",
        },
        {
            label: "Plannification",
            href: "/chefdtp/planing",
        },
    ],
};
