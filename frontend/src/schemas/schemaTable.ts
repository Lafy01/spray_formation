import { boolean, z } from "zod";

// We're keeping a simple non-relational schema here.
// IRL, you will have a schema for your data models.

// const userRoleSchema = z.enum(["employe", "chefdtp", "rh", "admin"]);
const userRoleSchema = z.enum(["apprenant", "formateur", "admin"]);

export const userWithoutDepartement = z.object({
    n_matricule: z.string().min(2, "Matricule must be at least 2 characters"),
    id_dep: z.string(),
    nom_empl: z.string().min(1, "Nom required"),
    prenom_empl: z.string(),
    email_empl: z.string().email("Email must be valid"),
    role: userRoleSchema,
});

export const departementSchemas = z.object({
    id_dep: z.string(),
    code_dep: z.string().min(1, "Code departement required"),
    nom_dep: z.string().min(1, "Nom required"),
    date_deb_form: z.string().optional(),
    date_fin_form: z.string().optional(),
    chef_dep: userWithoutDepartement.nullable(),
});

export const departementRegister = z.object({
    code_dep: z.string().min(1, "Code departement required"),
    nom_dep: z.string().min(1, "Nom required"),
    date_deb_form: z.string().optional(),
    date_fin_form: z.string().optional(),
    chef_dep: z.string().nullable(),
});

export const departementInfo = z.object({
    code_dep: z.string().min(1, "Code departement required"),
    nom_dep: z.string().min(1, "Nom required"),
    date_deb_form: z.string(),
    date_fin_form: z.string(),
})

export const soldeData = z.object({
    j_aqui_sld: z.string(),
    j_pris_sld: z.string().nullable(),
    j_reste_sld: z.string(),
    mois_sld: z.number(),
    annee_sld: z.number(),
})


export const departementUserInfo = z.object({
    code_dep: z.string().min(1, "Code departement required"),
    nom_dep: z.string().min(1, "Nom required"),
    date_deb_form: z.string().optional(),
    date_fin_form: z.string().optional(),
    chef_dep: userWithoutDepartement.nullable(),
})

export const userSchema = z.object({
    id_empl: z.string(),
    n_matricule: z.string().min(2, "Matricule must be at least 2 characters"),
    nom_empl: z.string().min(1, "Nom required"),
    prenom_empl: z.string(),
    email_empl: z.string().email("Email must be valid"),
    role: userRoleSchema,
    status: z.string().min(1, "Status required"),
    departement: departementUserInfo,
    solde: soldeData,
});


export const userRegister = z.object({
    n_matricule: z.string().min(2, "Le matricule doit comporter au moins 2 caractères"),
    id_dep: z.string().min(1, "Veuillez entrez sa Formation"),
    nom_empl: z.string().min(1, "Veuillez entrez son Nom"),
    prenom_empl: z.string(),
    email_empl: z.string().email("Veuillez entez son e-mail"),
    passw_empl: z.string().min(6, "Le mot de passe doit comporter au moins 6 caractères"),
    role: userRoleSchema,
});

export const congeSchema = z.object({
    id_empl: z.string().min(1, "Employe required"),
    motif_cong: z.string().min(1, "Motif required"),
    date_deb_cong: z
        .date({
            required_error: "Date debut required",
        })
        .min(new Date(), "Date debut doit etre au future"),
    debut_matin: boolean().default(true).optional(),
    nb_jour_conge: z.number({
        required_error: "Nb jour required",
        invalid_type_error: "Nb jour must be a number",    
    })
        .refine((value) => value % 1 === 0 || value % 1 === 0.5, {
            message: "Nombre de jour doit etre entier ou avec demi_journee",
        })
});

export const allCongeSchema = z.object({
    id_cong: z.string().min(1, "Conge required"),
    date_dmd_cong: z.string().min(1, "Date dmd required"),
    motif_cong: z.string().min(1, "Motif required"),
    date_deb_cong: z.string().min(1, "Date deb required"),
    date_fin_cong: z.string().min(1, "Date fin required"),
    nb_jour_cong: z.string(),
    status_cong: z.string().min(1, "Status required"),
    date_trait_cong: z.date({
        required_error: "Date trait required",
    }).nullable(),
    employe: userWithoutDepartement,
    departement: departementInfo,
});

export const myCongeSchema = z.object({
    motif_cong: z.string().min(1, "Motif required"),
    date_deb_cong: z.string(),
    debut_matin: z.boolean().default(true),
    nb_jour_cong: z.string()
        .transform(val => parseFloat(val))
        .refine(value => value % 0.5 === 0, {
            message: "The number of days must be a multiple of 0.5.",
        })
        .refine(value => value >= 0, {
            message: "The number of days must be a positive number.",
        })
});
export type myCongeType = z.infer<typeof myCongeSchema>;

export type userType = z.infer<typeof userSchema>;

export type userWithoutDepartementType = z.infer<typeof userWithoutDepartement>;

export type userRegisterType = z.infer<typeof userRegister>;

export type departementInfoType = z.infer<typeof departementInfo>;

export type departementType = z.infer<typeof departementSchemas>;

export type departementRegisterType = z.infer<typeof departementRegister>;

export type congeRegisterType = z.infer<typeof congeSchema>;

export type soldeDataType = z.infer<typeof soldeData>;

export type allCongeType = z.infer<typeof allCongeSchema>;