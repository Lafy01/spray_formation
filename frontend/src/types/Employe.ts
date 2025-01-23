
export type User = {
    id_empl: string;
    n_matricule: string;
    id_dep: string;
    id_suphier: string | null;
    nom_empl: string;
    prenom_empl: string;
    email_empl: string;
    passw_empl: string;
    role: UserRole;
    status: string;
    departement: DepartementUserInfo;
    solde: SoldeData,
}


export type PlanningUser = {
    id_empl: string;
    n_matricule: string;
    id_dep: string;
    id_suphier: string | null;
    nom_empl: string;
    prenom_empl: string;
    email_empl: string;
    passw_empl: string;
    role: UserRole;
    status: string;
    departement: DepartementUserInfo;
    solde: SoldeData,
    conge: AllCongePlanning[]
}


export type AllCongePlanning = {
    id_cong: string;
    date_dmd_cong: Date;
    motif_cong: string;
    date_deb_cong: string;
    date_fin_cong: string;
    nb_jour_cong: string;
    status_cong: string;
    date_trait_cong: Date | null;
}

export type UserWithoutDepartement = {
    n_matricule: string;
    id_dep: string;
    nom_empl: string;
    prenom_empl: string;
    email_empl: string;
    role: UserRole;
}

export type UserRole = 'apprenant' | 'formateur' | 'admin'

export type Departement = {
    id_dep: string;
    code_dep: string;
    nom_dep: string;
    date_deb_form: string;
    date_fin_form: string;
    chef_dep: UserWithoutDepartement | null;
}

export type DepartementUserInfo = {
    code_dep: string;
    nom_dep: string;
    date_deb_form: string;
    date_fin_form: string;
    chef_dep: UserWithoutDepartement | null;
}

export type DepartementInfo = {
    code_dep: string;
    nom_dep: string;
    date_deb_form: string;
    date_fin_form: string;
}

export type SoldeData = {
    j_aqui_sld: string;
    j_pris_sld: string;
    j_reste_sld: string;
    mois_sld: number;
    annee_sld: number;
}

export type AllUsers = {
    length: number,
    map(arg0: (user: any) => import("react").JSX.Element): import("react").ReactNode;
    allUser: AllUsers[];
}

export type AllConge = {
    id_cong: string;
    date_dmd_cong: Date;
    motif_cong: string;
    date_deb_cong: string;
    date_fin_cong: string;
    nb_jour_cong: string;
    status_cong: string;
    date_trait_cong: Date | null;
    employe: UserWithoutDepartement;
    departement: DepartementInfo;
}