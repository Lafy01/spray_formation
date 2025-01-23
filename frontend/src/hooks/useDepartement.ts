import { ApiResponse } from "@/types/ApiResponse";
import { Departement } from "@/types/Employe";
import axiosInstance, { apiUrl } from "@/utils/axiosInstance";
import { fetcher } from "@/utils/fetcher";
import { toast } from "sonner";
import useSWR, { mutate } from "swr";


export function useAllDepartement() {
    const {data: departements, error, isLoading} = useSWR<ApiResponse<Departement[]>>(`${apiUrl}/admin/secure/all_departement`,
        fetcher,
        {
            onError: (error) => {
                console.error("An error occurred during fetch", error)
                toast.error("Une erreur est survenue lors de la sélection des departements")
            },
        },
    );

    const updateDepartementData = async () => {
        try {
            await mutate(`${apiUrl}/admin/secure/all_departement`);
        } catch (error) {
            console.error("Error updating user data", error);
            toast.error("Unse erreur est survenue lors de la mise a jour de donne des departement");
        }
    }

    return {
        departements: departements?.data || [],
        isLoading,
        error: error?.message || null,
        updateDepartementData
    }
}

export const useAddDepartement = () => {

    const add = async (data: any) => {
        try {
            const res = await axiosInstance.post(`/admin/add_departement`, data);
            if (!res.data.success) return toast.error(res.data.message);
            toast.success("Success: ", res.data.message);
            return res.data;
        } catch (error) {
            console.error("An error occured during add departement");
            throw error;
        }
    }

    return { add };
}


export const useDeleteDepartement = () => {
    const { updateDepartementData } = useAllDepartement();
    const Delete = async (id_dep: string) => {
        try {
            const res = await axiosInstance.delete(`/admin/secure/delete_departement/${id_dep}`);
            
            if (!res.data.success) return toast.error(res.data.message);
            
            toast.success("Success: ", res.data.message);
            updateDepartementData();
            return res.data;
        } catch (error) {
            console.error("An error occured during delete departement");
            throw error;
        }
    }

    return { Delete };
}

export const useUpdateDepartement = () => {

    const updateDepartement= async (data: Departement) => {
        try {
            const res = await axiosInstance.post(`/admin/secure/update_departement`, data);
            if (!res.data.success) return toast.error(res.data.message);
            toast.success("Success: ", res.data.message);
            return res.data;
        } catch (error) {
            console.error("An error occured during update departement");
            throw error;
        }
    }
    return { updateDepartement };
}