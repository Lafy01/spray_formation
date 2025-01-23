"use client";
import { myCongeType } from "@/schemas/schemaTable";
import { ApiResponse } from "@/types/ApiResponse";
import { AllConge } from "@/types/Employe";
import axiosInstance, { apiUrl } from "@/utils/axiosInstance";
import { fetcher } from "@/utils/fetcher";
import { toast } from "sonner";
import useSWR, { mutate } from "swr";

export function useAllConge() {
    const { data: allConge, error, isLoading } = useSWR<ApiResponse<AllConge[]>>(`${apiUrl}/conge/all_conge`,
        fetcher,
        {
            onError: (error) => {
                console.error("An error occurred during fetch", error)
                toast.error("Une erreur est survenue lors de la sélection des conges")
            },
        },
    );

    const updateCongeData = async () => {
        try {
            await mutate(`${apiUrl}/conge/all_conge`);
        } catch (error) {
            console.error("Error updating user data", error);
            toast.error("Unse erreur est survenue lors de la mise a jour de donne des conges");
        }
    }

    return {
        conge: allConge?.data || [],
        isLoading,
        error: error?.message || null,
        updateCongeData
    }
}

export function useAllMyConge() {
    const { data: myConge, error, isLoading } = useSWR<ApiResponse<AllConge[]>>(`${apiUrl}/conge/secure/all_my_conge`,
        fetcher,
        {
            onError: (error) => {
                console.error("An error occurred during fetch", error)
                toast.error("Une erreur est survenue lors de la sélection des conges")
            },
        },
    );

    const updateMyCongeData = async () => {
        try {
            await mutate(`${apiUrl}/conge/secure/all_my_conge`);
        } catch (error) {
            console.error("Error updating user data", error);
            toast.error("Unse erreur est survenue lors de la mise a jour de donne des conges");
        }
    }

    return {
        myConge: myConge?.data || [],
        isLoading,
        error: error?.message || null,
        updateMyCongeData
    }
}

export function useAllMyEmployeConge() {
    const { data: myEmployeConge, error, isLoading } = useSWR<ApiResponse<AllConge[]>>(`${apiUrl}/conge/secure/all_my_employe_conge`,
        fetcher,
        {
            onError: (error) => {
                console.error("An error occurred during fetch", error)
                toast.error("Une erreur est survenue lors de la sélection des conges")
            },
        },
    );

    const updateMyEmployeCongeData = async () => {
        try {
            await mutate(`${apiUrl}/conge/secure/all_my_employe_conge`);
        } catch (error) {
            console.error("Error updating user data", error);
            toast.error("Unse erreur est survenue lors de la mise a jour de donne des conges");
        }
    }

    return {
        myEmployeConge: myEmployeConge?.data || [],
        isLoading,
        error: error?.message || null,
        updateMyEmployeCongeData
    }
}

export const useAddConge = () => {

    const add_my_conge = async (data: myCongeType) => {
        try {
            const res = await axiosInstance.post('/conge/secure/add_my_conge', data);
            if (!res.data.success) return toast.error(res.data.message);
            toast.success("Conge add Successfully: ", res.data.message);
            return res.data;
        } catch (error) {
            console.error("An error occured during add conge");
            throw error;
        }
    }

    return { add_my_conge };
}

export const useApproveConge = () => {

    const approveConge = async (id_cong: string) => {
        try {
            const res = await axiosInstance.put(`/conge/secure/approve_conge/${id_cong}`);
            if (!res.data.success) return toast.error(res.data.message);
            
            toast.success("Conge update Successfully: ", res.data.message);
            return res.data;
        } catch (error) {
            console.error("An error occured during update conge");
            throw error;
        }
    }

    return { approveConge };
}

export const useDeclineConge = () => {

    const declineConge = async (id_cong: string) => {
        try {
            const res = await axiosInstance.put(`/conge/secure/decline_conge/${id_cong}`);
            if (!res.data.success) return toast.error(res.data.message);
            
            toast.success("Conge decline Successfully: ", res.data.message);
            return res.data;
        } catch (error) {
            console.error("An error occured during update conge");
            throw error;
        }
    }

    return { declineConge };
}