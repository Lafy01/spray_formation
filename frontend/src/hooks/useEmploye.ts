import axiosInstance, { apiUrl } from "@/utils/axiosInstance";
import { toast } from "sonner";
import { useAllEmployes } from "./useAllEmploye";
import { userWithoutDepartementType } from "@/schemas/schemaTable";
import useSWR, { mutate } from "swr";
import { ApiResponse } from "@/types/ApiResponse";
import { User } from "@/types/Employe";
import { fetcher } from "@/utils/fetcher";

export const useMyProfile = () => {
    const {
        data: profile,
        error,
        isLoading,
    } = useSWR<ApiResponse<User>>(`${apiUrl}/employe/secure/me`, fetcher, {
        onError: (error) => {
            console.error("An error occurred during fetch", error);
            //         toast.error("Une erreur est survenue lors de la recuperation de votre profile: ")
        },
    });

    const updateProfileData = async () => {
        try {
            await mutate(`${apiUrl}/employe/secure/me`);
        } catch (error) {
            console.error("Error updating user data", error);
            toast.error(
                "Unse erreur est survenue lors de la mise a jour de mon profile"
            );
        }
    };

    return {
        myProfile: profile?.data,
        isLoading,
        error: error?.message || null,
        updateProfileData,
    };
};

export const useDeleteUser = () => {
    const { updateEmployeData } = useAllEmployes();
    const deleteUser = async (id: string) => {
        try {
            const res = await axiosInstance.delete(
                `employe/secure/delete_user/${id}`
            );
            if (!res.data.success) return toast.error(res.data.message);
            console.log("Delete successful: ", res);
            toast.success("Delete success: ", res.data?.message);
            updateEmployeData();
            return res.data;
        } catch (error) {
            console.error("An error occured during delete user");
            throw error;
        }
    };
    return { deleteUser };
};

export const useUpdateUser = () => {
    const { updateEmployeData } = useAllEmployes();
    const updateUser = async (
        id_empl: string,
        data: userWithoutDepartementType
    ) => {
        try {
            const res = await axiosInstance.post(
                `employe/secure/update_employe/${id_empl}`,
                data
            );

            if (!res.data.success) return toast.error(res.data.message);
            toast.success("Update success: ", res.data.message);
            updateEmployeData();
            return res.data;
        } catch (error) {
            console.error("Error updating user ", error);
            throw error;
        }
    };

    return { updateUser };
};
